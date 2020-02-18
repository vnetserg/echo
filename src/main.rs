use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

async fn serve_client(mut socket: TcpStream, addr: SocketAddr) {
    eprintln!("New connection from {}", addr);
    let mut buffer = [0u8; 1024];
    while let Ok(n) = socket.read(&mut buffer).await {
        if n == 0 {
            break;
        }
        let result = socket.write(&buffer[..n]).await;
        if result.is_err() {
            break;
        }
    }
    eprintln!("Connection dropped: {}", addr);
}

async fn run_server(mut listener: TcpListener) {
    while let Ok((socket, addr)) = listener.accept().await {
        tokio::spawn(serve_client(socket, addr));
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:33233";
    let listener = TcpListener::bind(addr).await.unwrap();
    run_server(listener).await;
}

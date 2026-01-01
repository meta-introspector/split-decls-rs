// SRC: ../rust/library/std/src/sys/net/mod.rs
/// This module contains the implementations of `TcpStream`, `TcpListener` and
/// `UdpSocket` as well as related functionality like DNS resolving.
mod connection;
pub use connection::*;

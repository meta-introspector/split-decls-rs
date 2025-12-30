// Generated macro for TcpStream (struct)
macro_rules! Depcrate_tcpTcpStream {
() => {
// Module: crate::tcp
// Provides: {"TcpStream"}
// Dependencies: {}
# [doc = " A TCP connection."] # [doc = ""] # [doc = " A [`TcpStream`] can be created by [`connect`][`TcpStream::connect()`]ing to an endpoint or by"] # [doc = " [`accept`][`TcpListener::accept()`]ing an incoming connection."] # [doc = ""] # [doc = " [`TcpStream`] is a bidirectional stream that implements traits [`AsyncRead`] and"] # [doc = " [`AsyncWrite`]."] # [doc = ""] # [doc = " Cloning a [`TcpStream`] creates another handle to the same socket. The socket will be closed"] # [doc = " when all handles to it are dropped. The reading and writing portions of the connection can also"] # [doc = " be shut down individually with the [`shutdown()`][`TcpStream::shutdown()`] method."] # [doc = ""] # [doc = " The Transmission Control Protocol is specified in [IETF RFC 793]."] # [doc = ""] # [doc = " [IETF RFC 793]: https://tools.ietf.org/html/rfc793"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use async_net::TcpStream;"] # [doc = " use futures_lite::prelude::*;"] # [doc = ""] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let mut stream = TcpStream::connect(\"127.0.0.1:8080\").await?;"] # [doc = " stream.write_all(b\"hello\").await?;"] # [doc = ""] # [doc = " let mut buf = vec![0u8; 1024];"] # [doc = " let n = stream.read(&mut buf).await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub struct TcpStream { inner : Arc < Async < std :: net :: TcpStream > > , readable : Option < async_io :: ReadableOwned < std :: net :: TcpStream > > , writable : Option < async_io :: WritableOwned < std :: net :: TcpStream > > , }
};
}

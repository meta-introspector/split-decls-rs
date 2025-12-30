// Generated macro for TcpListener (struct)
macro_rules! Depcrate_tcpTcpListener {
() => {
// Module: crate::tcp
// Provides: {"TcpListener"}
// Dependencies: {}
# [doc = " A TCP server, listening for connections."] # [doc = ""] # [doc = " After creating a [`TcpListener`] by [`bind`][`TcpListener::bind()`]ing it to an address, it"] # [doc = " listens for incoming TCP connections. These can be accepted by calling"] # [doc = " [`accept()`][`TcpListener::accept()`] or by awaiting items from the stream of"] # [doc = " [`incoming`][`TcpListener::incoming()`] connections."] # [doc = ""] # [doc = " Cloning a [`TcpListener`] creates another handle to the same socket. The socket will be closed"] # [doc = " when all handles to it are dropped."] # [doc = ""] # [doc = " The Transmission Control Protocol is specified in [IETF RFC 793]."] # [doc = ""] # [doc = " [IETF RFC 793]: https://tools.ietf.org/html/rfc793"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use async_net::TcpListener;"] # [doc = " use futures_lite::prelude::*;"] # [doc = ""] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let listener = TcpListener::bind(\"127.0.0.1:8080\").await?;"] # [doc = " let mut incoming = listener.incoming();"] # [doc = ""] # [doc = " while let Some(stream) = incoming.next().await {"] # [doc = "     let mut stream = stream?;"] # [doc = "     stream.write_all(b\"hello\").await?;"] # [doc = " }"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct TcpListener { inner : Arc < Async < std :: net :: TcpListener > > , }
};
}

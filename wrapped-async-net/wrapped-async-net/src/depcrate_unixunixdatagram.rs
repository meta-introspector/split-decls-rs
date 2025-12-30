// Generated macro for UnixDatagram (struct)
macro_rules! Depcrate_unixUnixDatagram {
() => {
// Module: crate::unix
// Provides: {"UnixDatagram"}
// Dependencies: {}
# [doc = " A Unix datagram socket."] # [doc = ""] # [doc = " After creating a [`UnixDatagram`] by [`bind`][`UnixDatagram::bind()`]ing it to a path, data can"] # [doc = " be [sent to] and [received from] any other socket address."] # [doc = ""] # [doc = " Cloning a [`UnixDatagram`] creates another handle to the same socket. The socket will be closed"] # [doc = " when all handles to it are dropped. The reading and writing portions of the socket can also be"] # [doc = " shut down individually with the [`shutdown()`][`UnixStream::shutdown()`] method."] # [doc = ""] # [doc = " [received from]: UnixDatagram::recv_from()"] # [doc = " [sent to]: UnixDatagram::send_to()"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use async_net::unix::UnixDatagram;"] # [doc = ""] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let socket = UnixDatagram::bind(\"/tmp/socket1\")?;"] # [doc = " socket.send_to(b\"hello\", \"/tmp/socket2\").await?;"] # [doc = ""] # [doc = " let mut buf = vec![0u8; 1024];"] # [doc = " let (n, addr) = socket.recv_from(&mut buf).await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct UnixDatagram { inner : Arc < Async < std :: os :: unix :: net :: UnixDatagram > > , }
};
}

// Generated macro for UnixStream (struct)
macro_rules! Depcrate_unixUnixStream {
() => {
// Module: crate::unix
// Provides: {"UnixStream"}
// Dependencies: {}
# [doc = " A Unix connection."] # [doc = ""] # [doc = " A [`UnixStream`] can be created by [`connect`][`UnixStream::connect()`]ing to an endpoint or by"] # [doc = " [`accept`][`UnixListener::accept()`]ing an incoming connection."] # [doc = ""] # [doc = " [`UnixStream`] is a bidirectional stream that implements traits [`AsyncRead`] and"] # [doc = " [`AsyncWrite`]."] # [doc = ""] # [doc = " Cloning a [`UnixStream`] creates another handle to the same socket. The socket will be closed"] # [doc = " when all handles to it are dropped. The reading and writing portions of the connection can also"] # [doc = " be shut down individually with the [`shutdown()`][`UnixStream::shutdown()`] method."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use async_net::unix::UnixStream;"] # [doc = " use futures_lite::prelude::*;"] # [doc = ""] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " let mut stream = UnixStream::connect(\"/tmp/socket\").await?;"] # [doc = " stream.write_all(b\"hello\").await?;"] # [doc = ""] # [doc = " let mut buf = vec![0u8; 1024];"] # [doc = " let n = stream.read(&mut buf).await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub struct UnixStream { inner : Arc < Async < std :: os :: unix :: net :: UnixStream > > , readable : Option < async_io :: ReadableOwned < std :: os :: unix :: net :: UnixStream > > , writable : Option < async_io :: WritableOwned < std :: os :: unix :: net :: UnixStream > > , }
};
}

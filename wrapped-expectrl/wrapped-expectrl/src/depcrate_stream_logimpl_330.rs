// Generated macro for impl_330 (impl)
macro_rules! Depcrate_stream_logimpl_330 {
() => {
// Module: crate::stream::log
// Provides: {"impl_330"}
// Dependencies: {}
# [cfg (feature = "async")] impl < S : AsyncWrite + Unpin , W : Write + Unpin > AsyncWrite for LogStream < S , W > { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < Result < usize > > { self . log_write (buf) ; Pin :: new (& mut self . get_mut () . stream) . poll_write (cx , buf) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { Pin :: new (& mut self . stream) . poll_flush (cx) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { Pin :: new (& mut self . stream) . poll_close (cx) } fn poll_write_vectored (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [io :: IoSlice < '_ >] ,) -> Poll < Result < usize > > { Pin :: new (& mut self . stream) . poll_write_vectored (cx , bufs) } }
};
}

// Generated macro for impl_139 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_139 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_139"}
// Dependencies: {}
# [cfg (feature = "tokio")] impl < R : tokio_dep :: io :: AsyncRead + tokio_dep :: io :: AsyncWrite > tokio_dep :: io :: AsyncWrite for BufReader < R > { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { self . get_pin_mut () . poll_write (cx , buf) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . get_pin_mut () . poll_flush (cx) } fn poll_shutdown (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . get_pin_mut () . poll_shutdown (cx) } }
};
}

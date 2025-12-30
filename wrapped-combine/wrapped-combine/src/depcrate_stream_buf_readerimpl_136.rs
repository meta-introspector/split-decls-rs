// Generated macro for impl_136 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_136 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_136"}
// Dependencies: {}
# [cfg (feature = "tokio-03")] impl < R : tokio_03_dep :: io :: AsyncRead + tokio_03_dep :: io :: AsyncWrite > tokio_03_dep :: io :: AsyncWrite for BufReader < R > { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { self . get_pin_mut () . poll_write (cx , buf) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . get_pin_mut () . poll_flush (cx) } fn poll_shutdown (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . get_pin_mut () . poll_shutdown (cx) } }
};
}

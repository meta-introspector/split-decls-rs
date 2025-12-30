// Generated macro for impl_133 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_133 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_133"}
// Dependencies: {}
# [cfg (feature = "tokio-02")] impl < R : tokio_02_dep :: io :: AsyncRead + tokio_02_dep :: io :: AsyncWrite > tokio_02_dep :: io :: AsyncWrite for BufReader < R > { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { self . get_pin_mut () . poll_write (cx , buf) } fn poll_write_buf < B : bytes_05 :: Buf > (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut B ,) -> Poll < io :: Result < usize > > { self . get_pin_mut () . poll_write_buf (cx , buf) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . get_pin_mut () . poll_flush (cx) } fn poll_shutdown (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . get_pin_mut () . poll_shutdown (cx) } }
};
}

// Generated macro for impl_173 (impl)
macro_rules! Depcrate_send_streamimpl_173 {
() => {
// Module: crate::send_stream
// Provides: {"impl_173"}
// Dependencies: {}
# [cfg (feature = "futures-io")] impl futures_io :: AsyncWrite for SendStream { fn poll_write (self : Pin < & mut Self > , cx : & mut Context , buf : & [u8]) -> Poll < io :: Result < usize > > { self . poll_write (cx , buf) . map_err (Into :: into) } fn poll_flush (self : Pin < & mut Self > , _cx : & mut Context) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } fn poll_close (self : Pin < & mut Self > , _cx : & mut Context) -> Poll < io :: Result < () > > { Poll :: Ready (self . get_mut () . finish () . map_err (Into :: into)) } }
};
}

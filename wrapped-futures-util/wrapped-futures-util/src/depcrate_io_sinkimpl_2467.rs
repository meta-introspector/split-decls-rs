// Generated macro for impl_2467 (impl)
macro_rules! Depcrate_io_sinkimpl_2467 {
() => {
// Module: crate::io::sink
// Provides: {"impl_2467"}
// Dependencies: {}
impl AsyncWrite for Sink { # [inline] fn poll_write (self : Pin < & mut Self > , _ : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (Ok (buf . len ())) } # [inline] fn poll_write_vectored (self : Pin < & mut Self > , _ : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (Ok (bufs . iter () . map (| b | b . len ()) . sum ())) } # [inline] fn poll_flush (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } # [inline] fn poll_close (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } }
};
}

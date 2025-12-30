// Generated macro for impl_289 (impl)
macro_rules! Depcrate_ioimpl_289 {
() => {
// Module: crate::io
// Provides: {"impl_289"}
// Dependencies: {}
impl AsyncWrite for Sink { # [inline] fn poll_write (self : Pin < & mut Self > , _ : & mut Context < '_ > , buf : & [u8]) -> Poll < Result < usize > > { Poll :: Ready (Ok (buf . len ())) } # [inline] fn poll_flush (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () > > { Poll :: Ready (Ok (())) } # [inline] fn poll_close (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () > > { Poll :: Ready (Ok (())) } }
};
}

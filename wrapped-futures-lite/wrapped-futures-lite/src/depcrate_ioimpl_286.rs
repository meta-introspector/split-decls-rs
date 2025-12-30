// Generated macro for impl_286 (impl)
macro_rules! Depcrate_ioimpl_286 {
() => {
// Module: crate::io
// Provides: {"impl_286"}
// Dependencies: {}
impl AsyncRead for Repeat { # [inline] fn poll_read (self : Pin < & mut Self > , _ : & mut Context < '_ > , buf : & mut [u8]) -> Poll < Result < usize > > { for b in & mut * buf { * b = self . byte ; } Poll :: Ready (Ok (buf . len ())) } }
};
}

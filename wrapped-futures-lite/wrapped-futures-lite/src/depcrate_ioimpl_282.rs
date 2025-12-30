// Generated macro for impl_282 (impl)
macro_rules! Depcrate_ioimpl_282 {
() => {
// Module: crate::io
// Provides: {"impl_282"}
// Dependencies: {}
impl AsyncRead for Empty { # [inline] fn poll_read (self : Pin < & mut Self > , _ : & mut Context < '_ > , _ : & mut [u8]) -> Poll < Result < usize > > { Poll :: Ready (Ok (0)) } }
};
}

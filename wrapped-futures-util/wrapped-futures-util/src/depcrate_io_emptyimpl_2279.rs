// Generated macro for impl_2279 (impl)
macro_rules! Depcrate_io_emptyimpl_2279 {
() => {
// Module: crate::io::empty
// Provides: {"impl_2279"}
// Dependencies: {}
impl AsyncRead for Empty { # [inline] fn poll_read (self : Pin < & mut Self > , _ : & mut Context < '_ > , _ : & mut [u8] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (Ok (0)) } }
};
}

// Generated macro for impl_2280 (impl)
macro_rules! Depcrate_io_emptyimpl_2280 {
() => {
// Module: crate::io::empty
// Provides: {"impl_2280"}
// Dependencies: {}
impl AsyncBufRead for Empty { # [inline] fn poll_fill_buf (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { Poll :: Ready (Ok (& [])) } # [inline] fn consume (self : Pin < & mut Self > , _ : usize) { } }
};
}

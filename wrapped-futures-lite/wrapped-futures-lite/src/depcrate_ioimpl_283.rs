// Generated macro for impl_283 (impl)
macro_rules! Depcrate_ioimpl_283 {
() => {
// Module: crate::io
// Provides: {"impl_283"}
// Dependencies: {}
impl AsyncBufRead for Empty { # [inline] fn poll_fill_buf < 'a > (self : Pin < & 'a mut Self > , _ : & mut Context < '_ >) -> Poll < Result < & 'a [u8] > > { Poll :: Ready (Ok (& [])) } # [inline] fn consume (self : Pin < & mut Self > , _ : usize) { } }
};
}

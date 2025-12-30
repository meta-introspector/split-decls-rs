// Generated macro for impl_18 (impl)
macro_rules! Depcrate_impls_slice_refimpl_18 {
() => {
// Module: crate::impls::slice_ref
// Provides: {"impl_18"}
// Dependencies: {}
impl BufRead for & [u8] { # [inline] async fn fill_buf (& mut self) -> Result < & [u8] , Self :: Error > { Ok (* self) } # [inline] fn consume (& mut self , amt : usize) { * self = & self [amt ..] ; } }
};
}

// Generated macro for impl_26 (impl)
macro_rules! Depcrate_impls_slice_refimpl_26 {
() => {
// Module: crate::impls::slice_ref
// Provides: {"impl_26"}
// Dependencies: {}
impl BufRead for & [u8] { # [inline] fn fill_buf (& mut self) -> Result < & [u8] , Self :: Error > { Ok (* self) } # [inline] fn consume (& mut self , amt : usize) { * self = & self [amt ..] ; } }
};
}

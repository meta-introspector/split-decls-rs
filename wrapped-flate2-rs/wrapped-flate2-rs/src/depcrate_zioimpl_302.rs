// Generated macro for impl_302 (impl)
macro_rules! Depcrate_zioimpl_302 {
() => {
// Module: crate::zio
// Provides: {"impl_302"}
// Dependencies: {}
impl < W : Write , D : Ops > Drop for Writer < W , D > { fn drop (& mut self) { if self . obj . is_some () { let _ = self . finish () ; } } }
};
}

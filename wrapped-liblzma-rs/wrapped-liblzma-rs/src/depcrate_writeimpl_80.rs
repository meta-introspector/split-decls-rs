// Generated macro for impl_80 (impl)
macro_rules! Depcrate_writeimpl_80 {
() => {
// Module: crate::write
// Provides: {"impl_80"}
// Dependencies: {}
impl < W : Write > Drop for XzEncoder < W > { # [inline] fn drop (& mut self) { if self . obj . is_some () { let _ = self . try_finish () ; } } }
};
}

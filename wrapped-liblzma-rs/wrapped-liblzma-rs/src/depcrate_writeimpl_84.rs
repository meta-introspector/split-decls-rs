// Generated macro for impl_84 (impl)
macro_rules! Depcrate_writeimpl_84 {
() => {
// Module: crate::write
// Provides: {"impl_84"}
// Dependencies: {}
impl < W : Write > Drop for XzDecoder < W > { # [inline] fn drop (& mut self) { if self . obj . is_some () { let _ = self . try_finish () ; } } }
};
}

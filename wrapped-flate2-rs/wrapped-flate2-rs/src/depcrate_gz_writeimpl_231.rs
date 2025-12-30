// Generated macro for impl_231 (impl)
macro_rules! Depcrate_gz_writeimpl_231 {
() => {
// Module: crate::gz::write
// Provides: {"impl_231"}
// Dependencies: {}
impl < W : Write > Drop for GzEncoder < W > { fn drop (& mut self) { if self . inner . is_present () { let _ = self . try_finish () ; } } }
};
}

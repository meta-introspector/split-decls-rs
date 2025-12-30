// Generated macro for impl_207 (impl)
macro_rules! Depcrate_writerimpl_207 {
() => {
// Module: crate::writer
// Provides: {"impl_207"}
// Dependencies: {}
impl < W : io :: Write > Drop for Writer < W > { fn drop (& mut self) { if self . wtr . is_some () && ! self . state . panicked { let _ = self . flush () ; } } }
};
}

// Generated macro for impl_50 (impl)
macro_rules! Depcrate_bitmapimpl_50 {
() => {
// Module: crate::bitmap
// Provides: {"impl_50"}
// Dependencies: {}
impl < P : PixelFormat > Drop for BitMapBackend < '_ , P > { fn drop (& mut self) { if ! self . saved { let _ = self . present () ; } } }
};
}

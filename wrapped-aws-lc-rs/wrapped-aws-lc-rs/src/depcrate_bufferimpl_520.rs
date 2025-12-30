// Generated macro for impl_520 (impl)
macro_rules! Depcrate_bufferimpl_520 {
() => {
// Module: crate::buffer
// Provides: {"impl_520"}
// Dependencies: {}
impl < T > Drop for Buffer < '_ , T > { fn drop (& mut self) { if let Cow :: Owned (b) = & mut self . 0 { b . zeroize () ; } } }
};
}

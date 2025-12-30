// Generated macro for impl_33 (impl)
macro_rules! Depcrate_bridge_bufferimpl_33 {
() => {
// Module: crate::bridge::buffer
// Provides: {"impl_33"}
// Dependencies: {}
impl Drop for Buffer { # [inline] fn drop (& mut self) { let b = self . take () ; (b . drop) (b) ; } }
};
}

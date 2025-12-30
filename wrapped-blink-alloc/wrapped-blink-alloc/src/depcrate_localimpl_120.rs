// Generated macro for impl_120 (impl)
macro_rules! Depcrate_localimpl_120 {
() => {
// Module: crate::local
// Provides: {"impl_120"}
// Dependencies: {}
impl < A > Drop for BlinkAlloc < A > where A : Allocator , { # [inline] fn drop (& mut self) { unsafe { self . arena . reset (false , & self . allocator) ; } } }
};
}

// Generated macro for impl_133 (impl)
macro_rules! Depcrate_syncimpl_133 {
() => {
// Module: crate::sync
// Provides: {"impl_133"}
// Dependencies: {}
impl < A : Allocator > Drop for SyncBlinkAlloc < A > { fn drop (& mut self) { unsafe { self . arena . reset (false , & self . allocator) ; } } }
};
}

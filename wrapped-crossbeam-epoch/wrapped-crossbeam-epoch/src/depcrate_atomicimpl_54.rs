// Generated macro for impl_54 (impl)
macro_rules! Depcrate_atomicimpl_54 {
() => {
// Module: crate::atomic
// Provides: {"impl_54"}
// Dependencies: {}
impl < T : ? Sized + Pointable > Drop for Owned < T > { fn drop (& mut self) { let (raw , _) = decompose_tag :: < T > (self . data) ; unsafe { T :: drop (raw) ; } } }
};
}

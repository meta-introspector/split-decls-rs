// Generated macro for impl_58 (impl)
macro_rules! Depcrate_atomicimpl_58 {
() => {
// Module: crate::atomic
// Provides: {"impl_58"}
// Dependencies: {}
impl < T : ? Sized + Pointable > DerefMut for Owned < T > { fn deref_mut (& mut self) -> & mut T { let (raw , _) = decompose_tag :: < T > (self . data) ; unsafe { T :: deref_mut (raw) } } }
};
}

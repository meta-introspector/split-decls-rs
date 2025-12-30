// Generated macro for impl_57 (impl)
macro_rules! Depcrate_atomicimpl_57 {
() => {
// Module: crate::atomic
// Provides: {"impl_57"}
// Dependencies: {}
impl < T : ? Sized + Pointable > Deref for Owned < T > { type Target = T ; fn deref (& self) -> & T { let (raw , _) = decompose_tag :: < T > (self . data) ; unsafe { T :: deref (raw) } } }
};
}

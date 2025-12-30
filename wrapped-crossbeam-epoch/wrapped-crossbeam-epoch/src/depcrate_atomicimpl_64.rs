// Generated macro for impl_64 (impl)
macro_rules! Depcrate_atomicimpl_64 {
() => {
// Module: crate::atomic
// Provides: {"impl_64"}
// Dependencies: {}
impl < T : ? Sized + Pointable > AsMut < T > for Owned < T > { fn as_mut (& mut self) -> & mut T { self . deref_mut () } }
};
}

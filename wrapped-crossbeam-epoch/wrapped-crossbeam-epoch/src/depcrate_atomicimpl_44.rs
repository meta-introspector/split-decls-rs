// Generated macro for impl_44 (impl)
macro_rules! Depcrate_atomicimpl_44 {
() => {
// Module: crate::atomic
// Provides: {"impl_44"}
// Dependencies: {}
impl < T > From < Box < T > > for Atomic < T > { fn from (b : Box < T >) -> Self { Self :: from (Owned :: from (b)) } }
};
}

// Generated macro for impl_56 (impl)
macro_rules! Depcrate_atomicimpl_56 {
() => {
// Module: crate::atomic
// Provides: {"impl_56"}
// Dependencies: {}
impl < T : Clone > Clone for Owned < T > { fn clone (& self) -> Self { Self :: new ((* * self) . clone ()) . with_tag (self . tag ()) } }
};
}

// Generated macro for impl_545 (impl)
macro_rules! Depcrate_spscimpl_545 {
() => {
// Module: crate::spsc
// Provides: {"impl_545"}
// Dependencies: {}
impl < T , const N : usize > Clone for Queue < T , N > where T : Clone , { fn clone (& self) -> Self { let mut new : Self = Self :: new () ; for s in self . iter () { unsafe { new . enqueue_unchecked (s . clone ()) ; } } new } }
};
}

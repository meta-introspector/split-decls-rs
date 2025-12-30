// Generated macro for impl_399 (impl)
macro_rules! Depcrate_hash_setimpl_399 {
() => {
// Module: crate::hash::set
// Provides: {"impl_399"}
// Dependencies: {}
impl < A , S > Clone for HashSet < A , S > where A : Clone , { # [doc = " Clone a set."] # [doc = ""] # [doc = " Time: O(1)"] # [inline] fn clone (& self) -> Self { HashSet { hasher : self . hasher . clone () , pool : self . pool . clone () , root : self . root . clone () , size : self . size , } } }
};
}

// Generated macro for impl_405 (impl)
macro_rules! Depcrate_hash_setimpl_405 {
() => {
// Module: crate::hash::set
// Provides: {"impl_405"}
// Dependencies: {}
impl < A , S > Default for HashSet < A , S > where S : BuildHasher + Default , { fn default () -> Self { let pool = HashSetPool :: default () ; let root = PoolRef :: default (& pool . 0) ; HashSet { hasher : Ref :: < S > :: default () , pool , root , size : 0 , } } }
};
}

// Generated macro for impl_330 (impl)
macro_rules! Depcrate_hash_mapimpl_330 {
() => {
// Module: crate::hash::map
// Provides: {"impl_330"}
// Dependencies: {}
impl < K , V , S > Default for HashMap < K , V , S > where S : BuildHasher + Default , { # [inline] fn default () -> Self { let pool = HashMapPool :: default () ; let root = PoolRef :: default (& pool . 0) ; HashMap { size : 0 , pool , root , hasher : Ref :: < S > :: default () , } } }
};
}

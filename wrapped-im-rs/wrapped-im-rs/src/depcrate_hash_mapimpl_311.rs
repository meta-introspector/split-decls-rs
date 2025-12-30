// Generated macro for impl_311 (impl)
macro_rules! Depcrate_hash_mapimpl_311 {
() => {
// Module: crate::hash::map
// Provides: {"impl_311"}
// Dependencies: {}
impl < K , V > HashMap < K , V , RandomState > { # [doc = " Construct an empty hash map."] # [inline] # [must_use] pub fn new () -> Self { Self :: default () } # [doc = " Construct an empty hash map using a specific memory pool."] # [cfg (feature = "pool")] # [must_use] pub fn with_pool (pool : & HashMapPool < K , V >) -> Self { let root = PoolRef :: default (& pool . 0) ; Self { size : 0 , hasher : Default :: default () , pool : pool . clone () , root , } } }
};
}

// Generated macro for impl_394 (impl)
macro_rules! Depcrate_hash_setimpl_394 {
() => {
// Module: crate::hash::set
// Provides: {"impl_394"}
// Dependencies: {}
impl < A > HashSet < A , RandomState > { # [doc = " Construct an empty set."] # [must_use] pub fn new () -> Self { Self :: default () } # [doc = " Construct an empty set using a specific memory pool."] # [cfg (feature = "pool")] # [must_use] pub fn with_pool (pool : & HashSetPool < A >) -> Self { Self { pool : pool . clone () , hasher : Default :: default () , size : 0 , root : PoolRef :: default (& pool . 0) , } } }
};
}

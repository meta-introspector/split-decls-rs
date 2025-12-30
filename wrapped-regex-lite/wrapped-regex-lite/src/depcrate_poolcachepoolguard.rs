// Generated macro for CachePoolGuard (type)
macro_rules! Depcrate_poolCachePoolGuard {
() => {
// Module: crate::pool
// Provides: {"CachePoolGuard"}
// Dependencies: {}
# [doc = " Same as above, but for the guard returned by a pool."] pub (crate) type CachePoolGuard < 'a > = PoolGuard < 'a , pikevm :: Cache , CachePoolFn > ;
};
}

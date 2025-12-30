// Generated macro for CachePool (type)
macro_rules! Depcrate_poolCachePool {
() => {
// Module: crate::pool
// Provides: {"CachePool"}
// Dependencies: {}
# [doc = " A type alias for our pool of meta::Cache that fixes the type parameters to"] # [doc = " what we use for the meta regex below."] pub (crate) type CachePool = Pool < pikevm :: Cache , CachePoolFn > ;
};
}

// Generated macro for StatementCache (struct)
macro_rules! Depcrate_cacheStatementCache {
() => {
// Module: crate::cache
// Provides: {"StatementCache"}
// Dependencies: {}
# [doc = " Prepared statements LRU cache."] # [derive (Debug)] pub struct StatementCache (RefCell < LruCache < Arc < str > , RawStatement > >) ;
};
}

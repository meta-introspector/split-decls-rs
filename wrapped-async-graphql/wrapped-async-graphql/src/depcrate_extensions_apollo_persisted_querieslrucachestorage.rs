// Generated macro for LruCacheStorage (struct)
macro_rules! Depcrate_extensions_apollo_persisted_queriesLruCacheStorage {
() => {
// Module: crate::extensions::apollo_persisted_queries
// Provides: {"LruCacheStorage"}
// Dependencies: {}
# [doc = " Memory-based LRU cache."] # [derive (Clone)] pub struct LruCacheStorage (Arc < Mutex < lru :: LruCache < String , ExecutableDocument > > >) ;
};
}

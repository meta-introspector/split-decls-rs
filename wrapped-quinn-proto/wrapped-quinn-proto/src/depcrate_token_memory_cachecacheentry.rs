// Generated macro for CacheEntry (struct)
macro_rules! Depcrate_token_memory_cacheCacheEntry {
() => {
// Module: crate::token_memory_cache
// Provides: {"CacheEntry"}
// Dependencies: {}
# [doc = " Cache entry within `TokenMemoryCache`'s LRU slab"] # [derive (Debug)] struct CacheEntry { server_name : Arc < str > , tokens : VecDeque < Bytes > , }
};
}

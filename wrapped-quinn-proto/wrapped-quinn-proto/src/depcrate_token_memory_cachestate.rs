// Generated macro for State (struct)
macro_rules! Depcrate_token_memory_cacheState {
() => {
// Module: crate::token_memory_cache
// Provides: {"State"}
// Dependencies: {}
# [doc = " Lockable inner state of `TokenMemoryCache`"] # [derive (Debug)] struct State { max_server_names : u32 , max_tokens_per_server : usize , lookup : HashMap < Arc < str > , u32 > , lru : LruSlab < CacheEntry > , }
};
}

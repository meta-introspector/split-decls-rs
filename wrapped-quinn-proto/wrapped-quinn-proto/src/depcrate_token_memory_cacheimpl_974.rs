// Generated macro for impl_974 (impl)
macro_rules! Depcrate_token_memory_cacheimpl_974 {
() => {
// Module: crate::token_memory_cache
// Provides: {"impl_974"}
// Dependencies: {}
impl CacheEntry { # [doc = " Construct with a single token"] fn new (server_name : Arc < str > , token : Bytes) -> Self { let mut tokens = VecDeque :: new () ; tokens . push_back (token) ; Self { server_name , tokens , } } }
};
}

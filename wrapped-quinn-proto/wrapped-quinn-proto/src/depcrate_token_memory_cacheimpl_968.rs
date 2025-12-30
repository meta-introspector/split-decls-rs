// Generated macro for impl_968 (impl)
macro_rules! Depcrate_token_memory_cacheimpl_968 {
() => {
// Module: crate::token_memory_cache
// Provides: {"impl_968"}
// Dependencies: {}
impl TokenMemoryCache { # [doc = " Construct empty"] pub fn new (max_server_names : u32 , max_tokens_per_server : usize) -> Self { Self (Mutex :: new (State :: new (max_server_names , max_tokens_per_server ,))) } }
};
}

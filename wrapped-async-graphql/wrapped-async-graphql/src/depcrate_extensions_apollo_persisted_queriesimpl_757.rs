// Generated macro for impl_757 (impl)
macro_rules! Depcrate_extensions_apollo_persisted_queriesimpl_757 {
() => {
// Module: crate::extensions::apollo_persisted_queries
// Provides: {"impl_757"}
// Dependencies: {}
impl LruCacheStorage { # [doc = " Creates a new LRU Cache that holds at most `cap` items."] pub fn new (cap : usize) -> Self { Self (Arc :: new (Mutex :: new (lru :: LruCache :: new (NonZeroUsize :: new (cap) . unwrap () ,)))) } }
};
}

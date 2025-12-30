// Generated macro for impl_518 (impl)
macro_rules! Depcrate_dataloader_cacheimpl_518 {
() => {
// Module: crate::dataloader::cache
// Provides: {"impl_518"}
// Dependencies: {}
impl CacheFactory for LruCache { fn create < K , V > (& self) -> Box < dyn CacheStorage < Key = K , Value = V > > where K : Send + Sync + Clone + Eq + Hash + 'static , V : Send + Sync + Clone + 'static , { Box :: new (LruCacheImpl (lru :: LruCache :: new (NonZeroUsize :: new (self . cap) . unwrap () ,))) } }
};
}

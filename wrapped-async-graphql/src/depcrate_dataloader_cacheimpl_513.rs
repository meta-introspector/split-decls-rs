// Generated macro for impl_513 (impl)
macro_rules! Depcrate_dataloader_cacheimpl_513 {
() => {
// Module: crate::dataloader::cache
// Provides: {"impl_513"}
// Dependencies: {}
impl < S : Send + Sync + BuildHasher + Default + 'static > CacheFactory for HashMapCache < S > { fn create < K , V > (& self) -> Box < dyn CacheStorage < Key = K , Value = V > > where K : Send + Sync + Clone + Eq + Hash + 'static , V : Send + Sync + Clone + 'static , { Box :: new (HashMapCacheImpl :: < K , V , S > (HashMap :: < K , V , S > :: default ())) } }
};
}

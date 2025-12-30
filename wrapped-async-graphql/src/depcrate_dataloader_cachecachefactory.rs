// Generated macro for CacheFactory (trait)
macro_rules! Depcrate_dataloader_cacheCacheFactory {
() => {
// Module: crate::dataloader::cache
// Provides: {"CacheFactory"}
// Dependencies: {}
# [doc = " Factory for creating cache storage."] pub trait CacheFactory : Send + Sync + 'static { # [doc = " Create a cache storage."] # [doc = ""] # [doc = " TODO: When GAT is stable, this memory allocation can be optimized away."] fn create < K , V > (& self) -> Box < dyn CacheStorage < Key = K , Value = V > > where K : Send + Sync + Clone + Eq + Hash + 'static , V : Send + Sync + Clone + 'static ; }
};
}

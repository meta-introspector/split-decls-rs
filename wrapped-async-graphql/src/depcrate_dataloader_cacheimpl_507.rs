// Generated macro for impl_507 (impl)
macro_rules! Depcrate_dataloader_cacheimpl_507 {
() => {
// Module: crate::dataloader::cache
// Provides: {"impl_507"}
// Dependencies: {}
impl CacheFactory for NoCache { fn create < K , V > (& self) -> Box < dyn CacheStorage < Key = K , Value = V > > where K : Send + Sync + Clone + Eq + Hash + 'static , V : Send + Sync + Clone + 'static , { Box :: new (NoCacheImpl { _mark1 : PhantomData , _mark2 : PhantomData , }) } }
};
}

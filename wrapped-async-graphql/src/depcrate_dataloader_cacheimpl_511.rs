// Generated macro for impl_511 (impl)
macro_rules! Depcrate_dataloader_cacheimpl_511 {
() => {
// Module: crate::dataloader::cache
// Provides: {"impl_511"}
// Dependencies: {}
impl < S : Send + Sync + BuildHasher + Default + 'static > HashMapCache < S > { # [doc = " Use specified `S: BuildHasher` to create a `HashMap` cache."] pub fn new () -> Self { Self { _mark : PhantomData } } }
};
}

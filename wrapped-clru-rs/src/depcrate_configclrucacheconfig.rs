// Generated macro for CLruCacheConfig (struct)
macro_rules! Depcrate_configCLruCacheConfig {
() => {
// Module: crate::config
// Provides: {"CLruCacheConfig"}
// Dependencies: {}
# [doc = " A configuration structure used to create an LRU cache."] pub struct CLruCacheConfig < K , V , S = RandomState , W = ZeroWeightScale > { pub (crate) capacity : NonZeroUsize , pub (crate) hash_builder : S , pub (crate) reserve : Option < usize > , pub (crate) scale : W , _marker : PhantomData < (K , V) > , }
};
}

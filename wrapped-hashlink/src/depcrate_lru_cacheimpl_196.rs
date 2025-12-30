// Generated macro for impl_196 (impl)
macro_rules! Depcrate_lru_cacheimpl_196 {
() => {
// Module: crate::lru_cache
// Provides: {"impl_196"}
// Dependencies: {}
impl < K : Eq + Hash , V > LruCache < K , V > { # [inline] pub fn new (capacity : usize) -> Self { LruCache { map : LinkedHashMap :: new () , max_size : capacity , } } # [doc = " Create a new unbounded `LruCache` that does not automatically evict entries."] # [doc = ""] # [doc = " A simple convenience method that is equivalent to `LruCache::new(usize::MAX)`"] # [inline] pub fn new_unbounded () -> Self { LruCache :: new (usize :: MAX) } }
};
}

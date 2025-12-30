// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl < K : Hash + Eq , V > LruCache < K , V > { # [doc = " Creates a new LRU Cache that holds at most `cap` items."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use lru::LruCache;"] # [doc = " use std::num::NonZeroUsize;"] # [doc = " let mut cache: LruCache<isize, &str> = LruCache::new(NonZeroUsize::new(10).unwrap());"] # [doc = " ```"] pub fn new (cap : NonZeroUsize) -> LruCache < K , V > { LruCache :: construct (cap , HashMap :: with_capacity (cap . get ())) } # [doc = " Creates a new LRU Cache that never automatically evicts items."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use lru::LruCache;"] # [doc = " use std::num::NonZeroUsize;"] # [doc = " let mut cache: LruCache<isize, &str> = LruCache::unbounded();"] # [doc = " ```"] pub fn unbounded () -> LruCache < K , V > { LruCache :: construct (NonZeroUsize :: MAX , HashMap :: default ()) } }
};
}

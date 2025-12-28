macro_rules! deps {
    () => {
        LruCacheImpl!();
        CacheFactory!();
        LruCache!();
        CacheStorage!();
    };
}

macro_rules! impl_374 {
    () => {
        deps!();
        impl CacheFactory for LruCache { fn create < K , V > (& self) -> Box < dyn CacheStorage < Key = K , Value = V > > where K : Send + Sync + Clone + Eq + Hash + 'static , V : Send + Sync + Clone + 'static , { Box :: new (LruCacheImpl (lru :: LruCache :: new (NonZeroUsize :: new (self . cap) . unwrap () ,))) } }
    };
}

impl_374!()
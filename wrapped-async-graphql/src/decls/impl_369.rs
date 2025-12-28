macro_rules! deps {
    () => {
        HashMapCache!();
        CacheFactory!();
        HashMapCacheImpl!();
        CacheStorage!();
    };
}

macro_rules! impl_369 {
    () => {
        deps!();
        impl < S : Send + Sync + BuildHasher + Default + 'static > CacheFactory for HashMapCache < S > { fn create < K , V > (& self) -> Box < dyn CacheStorage < Key = K , Value = V > > where K : Send + Sync + Clone + Eq + Hash + 'static , V : Send + Sync + Clone + 'static , { Box :: new (HashMapCacheImpl :: < K , V , S > (HashMap :: < K , V , S > :: default ())) } }
    };
}

impl_369!();
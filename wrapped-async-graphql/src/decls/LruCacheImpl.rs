macro_rules! deps {
    () => {
        LruCache!();
    };
}

macro_rules! LruCacheImpl {
    () => {
        deps!();
        struct LruCacheImpl < K , V > (lru :: LruCache < K , V >) ;
    };
}

LruCacheImpl!();
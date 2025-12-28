macro_rules! deps {
    () => {
        LruCache!();
    };
}

macro_rules! LruCacheStorage {
    () => {
        deps!();
        # [doc = " Memory-based LRU cache."] # [derive (Clone)] pub struct LruCacheStorage (Arc < Mutex < lru :: LruCache < String , ExecutableDocument > > >) ;
    };
}

LruCacheStorage!();
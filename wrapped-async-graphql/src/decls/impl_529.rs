macro_rules! deps {
    () => {
        LruCacheStorage!();
        LruCache!();
    };
}

macro_rules! impl_529 {
    () => {
        deps!();
        impl LruCacheStorage { # [doc = " Creates a new LRU Cache that holds at most `cap` items."] pub fn new (cap : usize) -> Self { Self (Arc :: new (Mutex :: new (lru :: LruCache :: new (NonZeroUsize :: new (cap) . unwrap () ,)))) } }
    };
}

impl_529!();
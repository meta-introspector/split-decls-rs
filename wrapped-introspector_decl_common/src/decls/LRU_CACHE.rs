macro_rules! LRU_CACHE {
    () => {
        pub static LRU_CACHE : Lazy < Mutex < LruCache < String , Vec < u8 > > > > = Lazy :: new (| | { let capacity = NonZeroUsize :: new (1024) . expect ("Cache capacity must be non-zero") ; Mutex :: new (LruCache :: new (capacity)) }) ;
    };
}

LRU_CACHE!();
macro_rules! CacheWrapper {
    () => {
        pub (crate) struct CacheWrapper { pub (crate) inner : NonNull < ffi :: rocksdb_cache_t > , }
    };
}

CacheWrapper!();
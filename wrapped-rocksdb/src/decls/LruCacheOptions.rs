macro_rules! LruCacheOptions {
    () => {
        pub struct LruCacheOptions { pub (crate) inner : * mut ffi :: rocksdb_lru_cache_options_t , }
    };
}

LruCacheOptions!();
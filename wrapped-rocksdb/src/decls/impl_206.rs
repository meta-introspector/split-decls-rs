macro_rules! deps {
    () => {
        LruCacheOptions!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl Default for LruCacheOptions { fn default () -> Self { let inner = unsafe { ffi :: rocksdb_lru_cache_options_create () } ; assert ! (! inner . is_null () , "Could not create RocksDB LRU cache options") ; Self { inner } } }
    };
}

impl_206!();
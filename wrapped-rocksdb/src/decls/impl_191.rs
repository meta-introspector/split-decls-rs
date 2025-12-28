macro_rules! deps {
    () => {
        LruCacheOptions!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl Drop for LruCacheOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_lru_cache_options_destroy (self . inner) ; } } }
    };
}

impl_191!()
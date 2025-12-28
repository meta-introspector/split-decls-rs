macro_rules! deps {
    () => {
        CacheWrapper!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl Drop for CacheWrapper { fn drop (& mut self) { unsafe { ffi :: rocksdb_cache_destroy (self . inner . as_ptr ()) ; } } }
    };
}

impl_148!()
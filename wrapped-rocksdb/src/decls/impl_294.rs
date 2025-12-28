macro_rules! deps {
    () => {
        MemoryUsage!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        impl Drop for MemoryUsage { fn drop (& mut self) { unsafe { ffi :: rocksdb_approximate_memory_usage_destroy (self . inner) ; } } }
    };
}

impl_294!()
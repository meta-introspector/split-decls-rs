macro_rules! deps {
    () => {
        MemoryUsageBuilder!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl Drop for MemoryUsageBuilder { fn drop (& mut self) { unsafe { ffi :: rocksdb_memory_consumers_destroy (self . inner) ; } } }
    };
}

impl_297!()
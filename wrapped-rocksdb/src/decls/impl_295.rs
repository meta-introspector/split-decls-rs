macro_rules! deps {
    () => {
        MemoryUsage!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl MemoryUsage { # [doc = " Approximate memory usage of all the mem-tables"] pub fn approximate_mem_table_total (& self) -> u64 { unsafe { ffi :: rocksdb_approximate_memory_usage_get_mem_table_total (self . inner) } } # [doc = " Approximate memory usage of un-flushed mem-tables"] pub fn approximate_mem_table_unflushed (& self) -> u64 { unsafe { ffi :: rocksdb_approximate_memory_usage_get_mem_table_unflushed (self . inner) } } # [doc = " Approximate memory usage of all the table readers"] pub fn approximate_mem_table_readers_total (& self) -> u64 { unsafe { ffi :: rocksdb_approximate_memory_usage_get_mem_table_readers_total (self . inner) } } # [doc = " Approximate memory usage by cache"] pub fn approximate_cache_total (& self) -> u64 { unsafe { ffi :: rocksdb_approximate_memory_usage_get_cache_total (self . inner) } } }
    };
}

impl_295!()
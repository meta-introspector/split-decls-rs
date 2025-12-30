// Generated macro for impl_367 (impl)
macro_rules! Depcrate_perfimpl_367 {
() => {
// Module: crate::perf
// Provides: {"impl_367"}
// Dependencies: {}
impl MemoryUsage { # [doc = " Approximate memory usage of all the mem-tables"] pub fn approximate_mem_table_total (& self) -> u64 { unsafe { ffi :: rocksdb_approximate_memory_usage_get_mem_table_total (self . inner) } } # [doc = " Approximate memory usage of un-flushed mem-tables"] pub fn approximate_mem_table_unflushed (& self) -> u64 { unsafe { ffi :: rocksdb_approximate_memory_usage_get_mem_table_unflushed (self . inner) } } # [doc = " Approximate memory usage of all the table readers"] pub fn approximate_mem_table_readers_total (& self) -> u64 { unsafe { ffi :: rocksdb_approximate_memory_usage_get_mem_table_readers_total (self . inner) } } # [doc = " Approximate memory usage by cache"] pub fn approximate_cache_total (& self) -> u64 { unsafe { ffi :: rocksdb_approximate_memory_usage_get_cache_total (self . inner) } } }
};
}

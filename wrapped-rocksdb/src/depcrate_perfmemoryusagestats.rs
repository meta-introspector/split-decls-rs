// Generated macro for MemoryUsageStats (struct)
macro_rules! Depcrate_perfMemoryUsageStats {
() => {
// Module: crate::perf
// Provides: {"MemoryUsageStats"}
// Dependencies: {}
# [doc = " Memory usage stats"] pub struct MemoryUsageStats { # [doc = " Approximate memory usage of all the mem-tables"] pub mem_table_total : u64 , # [doc = " Approximate memory usage of un-flushed mem-tables"] pub mem_table_unflushed : u64 , # [doc = " Approximate memory usage of all the table readers"] pub mem_table_readers_total : u64 , # [doc = " Approximate memory usage by cache"] pub cache_total : u64 , }
};
}

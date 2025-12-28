macro_rules! deps {
    () => {
        DB!();
    };
}

macro_rules! MemoryUsage {
    () => {
        deps!();
        # [doc = " Wrap over memory_usage_t. Hold current memory usage of the specified DB instances and caches"] pub struct MemoryUsage { inner : * mut ffi :: rocksdb_memory_usage_t , }
    };
}

MemoryUsage!()
macro_rules! deps {
    () => {
        MemoryUsage!();
    };
}

macro_rules! MemoryUsageBuilder {
    () => {
        deps!();
        # [doc = " Builder for MemoryUsage"] pub struct MemoryUsageBuilder { inner : * mut ffi :: rocksdb_memory_consumers_t , }
    };
}

MemoryUsageBuilder!();
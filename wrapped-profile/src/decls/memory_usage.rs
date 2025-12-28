macro_rules! memory_usage {
    () => {
        pub fn memory_usage () -> MemoryUsage { MemoryUsage :: now () }
    };
}

memory_usage!()
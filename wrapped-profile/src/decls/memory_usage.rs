macro_rules! deps {
    () => {
        MemoryUsage!();
    };
}

macro_rules! memory_usage {
    () => {
        deps!();
        pub fn memory_usage () -> MemoryUsage { MemoryUsage :: now () }
    };
}

memory_usage!()
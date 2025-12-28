macro_rules! deps {
    () => {
        Mmap!();
    };
}

macro_rules! MemoryMap {
    () => {
        deps!();
        # [allow (dead_code)] struct MemoryMap { sq_mmap : Mmap , sqe_mmap : Mmap , cq_mmap : Option < Mmap > , }
    };
}

MemoryMap!();
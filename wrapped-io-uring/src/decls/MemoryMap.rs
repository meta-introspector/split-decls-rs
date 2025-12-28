macro_rules! MemoryMap {
    () => {
        # [allow (dead_code)] struct MemoryMap { sq_mmap : Mmap , sqe_mmap : Mmap , cq_mmap : Option < Mmap > , }
    };
}

MemoryMap!()
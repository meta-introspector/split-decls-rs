macro_rules! deps {
    () => {
        EntryRange!();
    };
}

macro_rules! resolve_entry {
    () => {
        deps!();
        fn resolve_entry (range : data :: EntryRange , mapped_file : & memmap2 :: Mmap) -> Option < & [u8] > { mapped_file . get (range . start as usize .. range . end as usize) }
    };
}

resolve_entry!();
macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! SIZE_ALL_MEM_TABLES {
    () => {
        deps!();
        # [doc = " \"rocksdb.size-all-mem-tables\" - returns approximate size of active,"] # [doc = " unflushed immutable, and pinned immutable memtables (bytes)."] pub const SIZE_ALL_MEM_TABLES : & PropName = property ! ("size-all-mem-tables") ;
    };
}

SIZE_ALL_MEM_TABLES!()
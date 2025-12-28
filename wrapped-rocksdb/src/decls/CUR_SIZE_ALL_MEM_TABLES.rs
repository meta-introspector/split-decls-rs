macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! CUR_SIZE_ALL_MEM_TABLES {
    () => {
        deps!();
        # [doc = " \"rocksdb.cur-size-all-mem-tables\" - returns approximate size of active"] # [doc = " and unflushed immutable memtables (bytes)."] pub const CUR_SIZE_ALL_MEM_TABLES : & PropName = property ! ("cur-size-all-mem-tables") ;
    };
}

CUR_SIZE_ALL_MEM_TABLES!()
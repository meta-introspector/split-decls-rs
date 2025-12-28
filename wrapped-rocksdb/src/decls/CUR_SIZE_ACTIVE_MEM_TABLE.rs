macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! CUR_SIZE_ACTIVE_MEM_TABLE {
    () => {
        deps!();
        # [doc = " \"rocksdb.cur-size-active-mem-table\" - returns approximate size of active"] # [doc = " memtable (bytes)."] pub const CUR_SIZE_ACTIVE_MEM_TABLE : & PropName = property ! ("cur-size-active-mem-table") ;
    };
}

CUR_SIZE_ACTIVE_MEM_TABLE!()
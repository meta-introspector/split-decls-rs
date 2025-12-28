macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! NUM_DELETES_ACTIVE_MEM_TABLE {
    () => {
        deps!();
        # [doc = " \"rocksdb.num-deletes-active-mem-table\" - returns total number of delete"] # [doc = " entries in the active memtable."] pub const NUM_DELETES_ACTIVE_MEM_TABLE : & PropName = property ! ("num-deletes-active-mem-table") ;
    };
}

NUM_DELETES_ACTIVE_MEM_TABLE!();
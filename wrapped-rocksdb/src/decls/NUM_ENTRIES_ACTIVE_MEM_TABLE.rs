macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! NUM_ENTRIES_ACTIVE_MEM_TABLE {
    () => {
        deps!();
        # [doc = " \"rocksdb.num-entries-active-mem-table\" - returns total number of entries"] # [doc = " in the active memtable."] pub const NUM_ENTRIES_ACTIVE_MEM_TABLE : & PropName = property ! ("num-entries-active-mem-table") ;
    };
}

NUM_ENTRIES_ACTIVE_MEM_TABLE!()
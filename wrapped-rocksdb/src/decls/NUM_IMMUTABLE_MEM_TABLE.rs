macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! NUM_IMMUTABLE_MEM_TABLE {
    () => {
        deps!();
        # [doc = " \"rocksdb.num-immutable-mem-table\" - returns number of immutable"] # [doc = " memtables that have not yet been flushed."] pub const NUM_IMMUTABLE_MEM_TABLE : & PropName = property ! ("num-immutable-mem-table") ;
    };
}

NUM_IMMUTABLE_MEM_TABLE!();
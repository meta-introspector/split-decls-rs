macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! NUM_IMMUTABLE_MEM_TABLE_FLUSHED {
    () => {
        deps!();
        # [doc = " \"rocksdb.num-immutable-mem-table-flushed\" - returns number of immutable"] # [doc = " memtables that have already been flushed."] pub const NUM_IMMUTABLE_MEM_TABLE_FLUSHED : & PropName = property ! ("num-immutable-mem-table-flushed") ;
    };
}

NUM_IMMUTABLE_MEM_TABLE_FLUSHED!();
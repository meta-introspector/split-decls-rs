macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! MEM_TABLE_FLUSH_PENDING {
    () => {
        deps!();
        # [doc = " \"rocksdb.mem-table-flush-pending\" - returns 1 if a memtable flush is"] # [doc = " pending; otherwise, returns 0."] pub const MEM_TABLE_FLUSH_PENDING : & PropName = property ! ("mem-table-flush-pending") ;
    };
}

MEM_TABLE_FLUSH_PENDING!();
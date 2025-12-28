macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! ESTIMATE_TABLE_READERS_MEM {
    () => {
        deps!();
        # [doc = " \"rocksdb.estimate-table-readers-mem\" - returns estimated memory used for"] # [doc = " reading SST tables, excluding memory used in block cache (e.g.,"] # [doc = " filter and index blocks)."] pub const ESTIMATE_TABLE_READERS_MEM : & PropName = property ! ("estimate-table-readers-mem") ;
    };
}

ESTIMATE_TABLE_READERS_MEM!();
macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! NUM_DELETES_IMM_MEM_TABLES {
    () => {
        deps!();
        # [doc = " \"rocksdb.num-deletes-imm-mem-tables\" - returns total number of delete"] # [doc = " entries in the unflushed immutable memtables."] pub const NUM_DELETES_IMM_MEM_TABLES : & PropName = property ! ("num-deletes-imm-mem-tables") ;
    };
}

NUM_DELETES_IMM_MEM_TABLES!()
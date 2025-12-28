macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! NUM_ENTRIES_IMM_MEM_TABLES {
    () => {
        deps!();
        # [doc = " \"rocksdb.num-entries-imm-mem-tables\" - returns total number of entries"] # [doc = " in the unflushed immutable memtables."] pub const NUM_ENTRIES_IMM_MEM_TABLES : & PropName = property ! ("num-entries-imm-mem-tables") ;
    };
}

NUM_ENTRIES_IMM_MEM_TABLES!()
macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! COMPACTION_PENDING {
    () => {
        deps!();
        # [doc = " \"rocksdb.compaction-pending\" - returns 1 if at least one compaction is"] # [doc = " pending; otherwise, returns 0."] pub const COMPACTION_PENDING : & PropName = property ! ("compaction-pending") ;
    };
}

COMPACTION_PENDING!()
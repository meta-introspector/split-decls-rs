macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! ESTIMATE_PENDING_COMPACTION_BYTES {
    () => {
        deps!();
        # [doc = " \"rocksdb.estimate-pending-compaction-bytes\" - returns estimated total"] # [doc = " number of bytes compaction needs to rewrite to get all levels down"] # [doc = " to under target size. Not valid for other compactions than level-"] # [doc = " based."] pub const ESTIMATE_PENDING_COMPACTION_BYTES : & PropName = property ! ("estimate-pending-compaction-bytes") ;
    };
}

ESTIMATE_PENDING_COMPACTION_BYTES!();
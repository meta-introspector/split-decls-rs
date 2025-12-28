macro_rules! deps {
    () => {
        PropName!();
        DB!();
    };
}

macro_rules! ESTIMATE_OLDEST_KEY_TIME {
    () => {
        deps!();
        # [doc = " \"rocksdb.estimate-oldest-key-time\" - returns an estimation of"] # [doc = " oldest key timestamp in the DB. Currently only available for"] # [doc = " FIFO compaction with"] # [doc = " compaction_options_fifo.allow_compaction = false."] pub const ESTIMATE_OLDEST_KEY_TIME : & PropName = property ! ("estimate-oldest-key-time") ;
    };
}

ESTIMATE_OLDEST_KEY_TIME!()
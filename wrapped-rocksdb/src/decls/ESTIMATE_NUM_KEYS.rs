macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! ESTIMATE_NUM_KEYS {
    () => {
        deps!();
        # [doc = " \"rocksdb.estimate-num-keys\" - returns estimated number of total keys in"] # [doc = " the active and unflushed immutable memtables and storage."] pub const ESTIMATE_NUM_KEYS : & PropName = property ! ("estimate-num-keys") ;
    };
}

ESTIMATE_NUM_KEYS!();
macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! OLDEST_SNAPSHOT_TIME {
    () => {
        deps!();
        # [doc = " \"rocksdb.oldest-snapshot-time\" - returns number representing unix"] # [doc = " timestamp of oldest unreleased snapshot."] pub const OLDEST_SNAPSHOT_TIME : & PropName = property ! ("oldest-snapshot-time") ;
    };
}

OLDEST_SNAPSHOT_TIME!();
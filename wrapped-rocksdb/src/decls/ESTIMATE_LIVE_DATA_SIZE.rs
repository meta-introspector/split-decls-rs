macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! ESTIMATE_LIVE_DATA_SIZE {
    () => {
        deps!();
        # [doc = " \"rocksdb.estimate-live-data-size\" - returns an estimate of the amount of"] # [doc = " live data in bytes."] pub const ESTIMATE_LIVE_DATA_SIZE : & PropName = property ! ("estimate-live-data-size") ;
    };
}

ESTIMATE_LIVE_DATA_SIZE!()
macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! ACTUAL_DELAYED_WRITE_RATE {
    () => {
        deps!();
        # [doc = " \"rocksdb.actual-delayed-write-rate\" - returns the current actual delayed"] # [doc = " write rate. 0 means no delay."] pub const ACTUAL_DELAYED_WRITE_RATE : & PropName = property ! ("actual-delayed-write-rate") ;
    };
}

ACTUAL_DELAYED_WRITE_RATE!()
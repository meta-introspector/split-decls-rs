macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! CFSTATS {
    () => {
        deps!();
        # [doc = " \"rocksdb.cfstats\" - Both of \"rocksdb.cfstats-no-file-histogram\" and"] # [doc = " \"rocksdb.cf-file-histogram\" together. See below for description"] # [doc = " of the two."] pub const CFSTATS : & PropName = property ! ("CFSTATS") ;
    };
}

CFSTATS!();
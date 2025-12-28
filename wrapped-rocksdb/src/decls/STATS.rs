macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! STATS {
    () => {
        deps!();
        # [doc = " \"rocksdb.stats\" - returns a multi-line string containing the data"] # [doc = " described by kCFStats followed by the data described by kDBStats."] pub const STATS : & PropName = property ! ("stats") ;
    };
}

STATS!();
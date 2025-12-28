macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! LEVELSTATS {
    () => {
        deps!();
        # [doc = " \"rocksdb.levelstats\" - returns multi-line string containing the number"] # [doc = " of files per level and total size of each level (MB)."] pub const LEVELSTATS : & PropName = property ! ("levelstats") ;
    };
}

LEVELSTATS!();
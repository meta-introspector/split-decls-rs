macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! SSTABLES {
    () => {
        deps!();
        # [doc = " \"rocksdb.sstables\" - returns a multi-line string summarizing current"] # [doc = " SST files."] pub const SSTABLES : & PropName = property ! ("sstables") ;
    };
}

SSTABLES!();
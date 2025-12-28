macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! NUM_SNAPSHOTS {
    () => {
        deps!();
        # [doc = " \"rocksdb.num-snapshots\" - returns number of unreleased snapshots of the"] # [doc = " database."] pub const NUM_SNAPSHOTS : & PropName = property ! ("num-snapshots") ;
    };
}

NUM_SNAPSHOTS!();
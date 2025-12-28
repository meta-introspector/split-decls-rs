macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! BASE_LEVEL {
    () => {
        deps!();
        # [doc = " \"rocksdb.base-level\" - returns number of level to which L0 data will be"] # [doc = " compacted."] pub const BASE_LEVEL : & PropName = property ! ("base-level") ;
    };
}

BASE_LEVEL!()
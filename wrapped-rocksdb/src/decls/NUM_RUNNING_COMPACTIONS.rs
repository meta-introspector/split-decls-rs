macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! NUM_RUNNING_COMPACTIONS {
    () => {
        deps!();
        # [doc = " \"rocksdb.num-running-compactions\" - returns the number of currently"] # [doc = " running compactions."] pub const NUM_RUNNING_COMPACTIONS : & PropName = property ! ("num-running-compactions") ;
    };
}

NUM_RUNNING_COMPACTIONS!()
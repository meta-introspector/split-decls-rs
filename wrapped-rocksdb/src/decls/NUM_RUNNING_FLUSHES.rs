macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! NUM_RUNNING_FLUSHES {
    () => {
        deps!();
        # [doc = " \"rocksdb.num-running-flushes\" - returns the number of currently running"] # [doc = " flushes."] pub const NUM_RUNNING_FLUSHES : & PropName = property ! ("num-running-flushes") ;
    };
}

NUM_RUNNING_FLUSHES!();
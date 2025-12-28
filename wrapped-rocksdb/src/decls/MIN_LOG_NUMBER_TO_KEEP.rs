macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! MIN_LOG_NUMBER_TO_KEEP {
    () => {
        deps!();
        # [doc = " \"rocksdb.min-log-number-to-keep\" - return the minimum log number of the"] # [doc = " log files that should be kept."] pub const MIN_LOG_NUMBER_TO_KEEP : & PropName = property ! ("min-log-number-to-keep") ;
    };
}

MIN_LOG_NUMBER_TO_KEEP!();
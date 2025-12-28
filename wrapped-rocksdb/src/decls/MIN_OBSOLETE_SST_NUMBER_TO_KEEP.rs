macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! MIN_OBSOLETE_SST_NUMBER_TO_KEEP {
    () => {
        deps!();
        # [doc = " \"rocksdb.min-obsolete-sst-number-to-keep\" - return the minimum file"] # [doc = " number for an obsolete SST to be kept. The max value of `uint64_t`"] # [doc = " will be returned if all obsolete files can be deleted."] pub const MIN_OBSOLETE_SST_NUMBER_TO_KEEP : & PropName = property ! ("min-obsolete-sst-number-to-keep") ;
    };
}

MIN_OBSOLETE_SST_NUMBER_TO_KEEP!();
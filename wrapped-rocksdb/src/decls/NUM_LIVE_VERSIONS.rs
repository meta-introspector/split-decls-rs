macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! NUM_LIVE_VERSIONS {
    () => {
        deps!();
        # [doc = " \"rocksdb.num-live-versions\" - returns number of live versions. `Version`"] # [doc = " is an internal data structure. See version_set.h for details. More"] # [doc = " live versions often mean more SST files are held from being deleted,"] # [doc = " by iterators or unfinished compactions."] pub const NUM_LIVE_VERSIONS : & PropName = property ! ("num-live-versions") ;
    };
}

NUM_LIVE_VERSIONS!();
macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! LIVE_SST_FILES_SIZE {
    () => {
        deps!();
        # [doc = " \"rocksdb.live-sst-files-size\" - returns total size (bytes) of all SST"] # [doc = " files belong to the latest LSM tree."] pub const LIVE_SST_FILES_SIZE : & PropName = property ! ("live-sst-files-size") ;
    };
}

LIVE_SST_FILES_SIZE!()
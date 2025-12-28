macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! TOTAL_SST_FILES_SIZE {
    () => {
        deps!();
        # [doc = " \"rocksdb.total-sst-files-size\" - returns total size (bytes) of all SST"] # [doc = " files."] # [doc = " WARNING: may slow down online queries if there are too many files."] pub const TOTAL_SST_FILES_SIZE : & PropName = property ! ("total-sst-files-size") ;
    };
}

TOTAL_SST_FILES_SIZE!();
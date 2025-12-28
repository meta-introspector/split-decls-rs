macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! IS_FILE_DELETIONS_ENABLED {
    () => {
        deps!();
        # [doc = " \"rocksdb.is-file-deletions-enabled\" - returns 0 if deletion of obsolete"] # [doc = " files is enabled; otherwise, returns a non-zero number."] pub const IS_FILE_DELETIONS_ENABLED : & PropName = property ! ("is-file-deletions-enabled") ;
    };
}

IS_FILE_DELETIONS_ENABLED!();
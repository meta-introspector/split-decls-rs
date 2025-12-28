macro_rules! DBPath {
    () => {
        # [doc = " Represents a path where sst files can be put into"] pub struct DBPath { pub (crate) inner : * mut ffi :: rocksdb_dbpath_t , }
    };
}

DBPath!();
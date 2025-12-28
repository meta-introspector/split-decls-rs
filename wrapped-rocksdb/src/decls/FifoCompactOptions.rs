macro_rules! FifoCompactOptions {
    () => {
        pub struct FifoCompactOptions { pub (crate) inner : * mut ffi :: rocksdb_fifo_compaction_options_t , }
    };
}

FifoCompactOptions!();
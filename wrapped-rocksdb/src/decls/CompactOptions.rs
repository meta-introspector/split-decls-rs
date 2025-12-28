macro_rules! CompactOptions {
    () => {
        pub struct CompactOptions { pub (crate) inner : * mut ffi :: rocksdb_compactoptions_t , full_history_ts_low : Option < Vec < u8 > > , }
    };
}

CompactOptions!();
macro_rules! UniversalCompactOptions {
    () => {
        pub struct UniversalCompactOptions { pub (crate) inner : * mut ffi :: rocksdb_universal_compaction_options_t , }
    };
}

UniversalCompactOptions!()
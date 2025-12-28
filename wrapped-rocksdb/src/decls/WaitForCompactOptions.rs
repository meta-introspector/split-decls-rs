macro_rules! WaitForCompactOptions {
    () => {
        pub struct WaitForCompactOptions { pub (crate) inner : * mut ffi :: rocksdb_wait_for_compact_options_t , }
    };
}

WaitForCompactOptions!()
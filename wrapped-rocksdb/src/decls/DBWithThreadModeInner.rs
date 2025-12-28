macro_rules! DBWithThreadModeInner {
    () => {
        pub struct DBWithThreadModeInner { inner : * mut ffi :: rocksdb_t , }
    };
}

DBWithThreadModeInner!()
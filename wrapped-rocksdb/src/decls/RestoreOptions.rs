macro_rules! RestoreOptions {
    () => {
        pub struct RestoreOptions { inner : * mut ffi :: rocksdb_restore_options_t , }
    };
}

RestoreOptions!()
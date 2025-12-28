macro_rules! destroy_handle {
    () => {
        fn destroy_handle (handle : * mut ffi :: rocksdb_column_family_handle_t) { unsafe { ffi :: rocksdb_column_family_handle_destroy (handle) ; } }
    };
}

destroy_handle!();
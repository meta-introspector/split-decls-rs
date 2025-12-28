macro_rules! UnboundColumnFamily {
    () => {
        pub (crate) struct UnboundColumnFamily { pub (crate) inner : * mut ffi :: rocksdb_column_family_handle_t , }
    };
}

UnboundColumnFamily!()
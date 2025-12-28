macro_rules! OptimisticTransactionDBInner {
    () => {
        pub struct OptimisticTransactionDBInner { base : * mut ffi :: rocksdb_t , db : * mut ffi :: rocksdb_optimistictransactiondb_t , }
    };
}

OptimisticTransactionDBInner!();
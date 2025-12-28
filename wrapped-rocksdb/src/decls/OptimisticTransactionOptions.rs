macro_rules! OptimisticTransactionOptions {
    () => {
        pub struct OptimisticTransactionOptions { pub (crate) inner : * mut ffi :: rocksdb_optimistictransaction_options_t , }
    };
}

OptimisticTransactionOptions!()
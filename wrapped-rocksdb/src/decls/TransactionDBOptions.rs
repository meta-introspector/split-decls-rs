macro_rules! TransactionDBOptions {
    () => {
        pub struct TransactionDBOptions { pub (crate) inner : * mut ffi :: rocksdb_transactiondb_options_t , }
    };
}

TransactionDBOptions!()
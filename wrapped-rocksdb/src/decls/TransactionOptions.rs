macro_rules! TransactionOptions {
    () => {
        pub struct TransactionOptions { pub (crate) inner : * mut ffi :: rocksdb_transaction_options_t , }
    };
}

TransactionOptions!();
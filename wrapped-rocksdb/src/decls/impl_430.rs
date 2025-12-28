macro_rules! deps {
    () => {
        TransactionOptions!();
    };
}

macro_rules! impl_430 {
    () => {
        deps!();
        impl Default for TransactionOptions { fn default () -> Self { let txn_opts = unsafe { ffi :: rocksdb_transaction_options_create () } ; assert ! (! txn_opts . is_null () , "Could not create RocksDB transaction options") ; Self { inner : txn_opts } } }
    };
}

impl_430!()
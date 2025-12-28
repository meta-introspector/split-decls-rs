macro_rules! deps {
    () => {
        TransactionDBOptions!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        impl Default for TransactionDBOptions { fn default () -> Self { let txn_db_opts = unsafe { ffi :: rocksdb_transactiondb_options_create () } ; assert ! (! txn_db_opts . is_null () , "Could not create RocksDB transaction_db options") ; Self { inner : txn_db_opts } } }
    };
}

impl_436!()
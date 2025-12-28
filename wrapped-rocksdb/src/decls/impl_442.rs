macro_rules! deps {
    () => {
        OptimisticTransactionOptions!();
    };
}

macro_rules! impl_442 {
    () => {
        deps!();
        impl Default for OptimisticTransactionOptions { fn default () -> Self { let txn_opts = unsafe { ffi :: rocksdb_optimistictransaction_options_create () } ; assert ! (! txn_opts . is_null () , "Could not create RocksDB optimistic transaction options") ; Self { inner : txn_opts } } }
    };
}

impl_442!()
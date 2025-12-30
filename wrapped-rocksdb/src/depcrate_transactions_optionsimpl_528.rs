// Generated macro for impl_528 (impl)
macro_rules! Depcrate_transactions_optionsimpl_528 {
() => {
// Module: crate::transactions::options
// Provides: {"impl_528"}
// Dependencies: {}
impl Default for TransactionDBOptions { fn default () -> Self { let txn_db_opts = unsafe { ffi :: rocksdb_transactiondb_options_create () } ; assert ! (! txn_db_opts . is_null () , "Could not create RocksDB transaction_db options") ; Self { inner : txn_db_opts } } }
};
}

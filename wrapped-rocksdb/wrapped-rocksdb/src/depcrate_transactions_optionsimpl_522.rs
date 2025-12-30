// Generated macro for impl_522 (impl)
macro_rules! Depcrate_transactions_optionsimpl_522 {
() => {
// Module: crate::transactions::options
// Provides: {"impl_522"}
// Dependencies: {}
impl Default for TransactionOptions { fn default () -> Self { let txn_opts = unsafe { ffi :: rocksdb_transaction_options_create () } ; assert ! (! txn_opts . is_null () , "Could not create RocksDB transaction options") ; Self { inner : txn_opts } } }
};
}

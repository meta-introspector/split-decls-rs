// Generated macro for impl_534 (impl)
macro_rules! Depcrate_transactions_optionsimpl_534 {
() => {
// Module: crate::transactions::options
// Provides: {"impl_534"}
// Dependencies: {}
impl Default for OptimisticTransactionOptions { fn default () -> Self { let txn_opts = unsafe { ffi :: rocksdb_optimistictransaction_options_create () } ; assert ! (! txn_opts . is_null () , "Could not create RocksDB optimistic transaction options") ; Self { inner : txn_opts } } }
};
}

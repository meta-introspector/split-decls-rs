// Generated macro for impl_563 (impl)
macro_rules! Depcrate_transactions_transaction_dbimpl_563 {
() => {
// Module: crate::transactions::transaction_db
// Provides: {"impl_563"}
// Dependencies: {}
impl < T : ThreadMode > Drop for TransactionDB < T > { fn drop (& mut self) { unsafe { self . prepared_transactions () . clear () ; self . cfs . drop_all_cfs_internal () ; ffi :: rocksdb_transactiondb_close (self . inner) ; } } }
};
}

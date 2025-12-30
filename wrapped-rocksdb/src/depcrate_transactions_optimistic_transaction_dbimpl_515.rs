// Generated macro for impl_515 (impl)
macro_rules! Depcrate_transactions_optimistic_transaction_dbimpl_515 {
() => {
// Module: crate::transactions::optimistic_transaction_db
// Provides: {"impl_515"}
// Dependencies: {}
impl Drop for OptimisticTransactionDBInner { fn drop (& mut self) { unsafe { ffi :: rocksdb_optimistictransactiondb_close_base_db (self . base) ; ffi :: rocksdb_optimistictransactiondb_close (self . db) ; } } }
};
}

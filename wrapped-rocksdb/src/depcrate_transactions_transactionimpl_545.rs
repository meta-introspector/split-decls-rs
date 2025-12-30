// Generated macro for impl_545 (impl)
macro_rules! Depcrate_transactions_transactionimpl_545 {
() => {
// Module: crate::transactions::transaction
// Provides: {"impl_545"}
// Dependencies: {}
impl < DB > Drop for Transaction < '_ , DB > { fn drop (& mut self) { unsafe { ffi :: rocksdb_transaction_destroy (self . inner) ; } } }
};
}

// Generated macro for impl_524 (impl)
macro_rules! Depcrate_transactions_optionsimpl_524 {
() => {
// Module: crate::transactions::options
// Provides: {"impl_524"}
// Dependencies: {}
impl Drop for TransactionOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_transaction_options_destroy (self . inner) ; } } }
};
}

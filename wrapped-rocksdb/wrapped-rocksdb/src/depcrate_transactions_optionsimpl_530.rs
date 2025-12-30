// Generated macro for impl_530 (impl)
macro_rules! Depcrate_transactions_optionsimpl_530 {
() => {
// Module: crate::transactions::options
// Provides: {"impl_530"}
// Dependencies: {}
impl Drop for TransactionDBOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_transactiondb_options_destroy (self . inner) ; } } }
};
}

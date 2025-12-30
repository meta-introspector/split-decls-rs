// Generated macro for impl_536 (impl)
macro_rules! Depcrate_transactions_optionsimpl_536 {
() => {
// Module: crate::transactions::options
// Provides: {"impl_536"}
// Dependencies: {}
impl Drop for OptimisticTransactionOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_optimistictransaction_options_destroy (self . inner) ; } } }
};
}

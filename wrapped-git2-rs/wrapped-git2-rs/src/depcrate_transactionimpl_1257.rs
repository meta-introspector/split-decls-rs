// Generated macro for impl_1257 (impl)
macro_rules! Depcrate_transactionimpl_1257 {
() => {
// Module: crate::transaction
// Provides: {"impl_1257"}
// Dependencies: {}
impl Drop for Transaction < '_ > { fn drop (& mut self) { unsafe { raw :: git_transaction_free (self . raw) } } }
};
}

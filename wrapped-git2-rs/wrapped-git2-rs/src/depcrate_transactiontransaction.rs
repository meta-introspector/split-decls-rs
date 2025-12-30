// Generated macro for Transaction (struct)
macro_rules! Depcrate_transactionTransaction {
() => {
// Module: crate::transaction
// Provides: {"Transaction"}
// Dependencies: {}
# [doc = " A structure representing a transactional update of a repository's references."] # [doc = ""] # [doc = " Transactions work by locking loose refs for as long as the [`Transaction`]"] # [doc = " is held, and committing all changes to disk when [`Transaction::commit`] is"] # [doc = " called. Note that committing is not atomic: if an operation fails, the"] # [doc = " transaction aborts, but previous successful operations are not rolled back."] pub struct Transaction < 'repo > { raw : * mut raw :: git_transaction , _marker : marker :: PhantomData < & 'repo Repository > , }
};
}

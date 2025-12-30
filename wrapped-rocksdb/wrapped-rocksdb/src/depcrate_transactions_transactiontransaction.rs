// Generated macro for Transaction (struct)
macro_rules! Depcrate_transactions_transactionTransaction {
() => {
// Module: crate::transactions::transaction
// Provides: {"Transaction"}
// Dependencies: {}
# [doc = " RocksDB Transaction."] # [doc = ""] # [doc = " To use transactions, you must first create a [`TransactionDB`] or [`OptimisticTransactionDB`]."] # [doc = ""] # [doc = " [`TransactionDB`]: crate::TransactionDB"] # [doc = " [`OptimisticTransactionDB`]: crate::OptimisticTransactionDB"] pub struct Transaction < 'db , DB > { pub (crate) inner : * mut ffi :: rocksdb_transaction_t , pub (crate) _marker : PhantomData < & 'db DB > , }
};
}

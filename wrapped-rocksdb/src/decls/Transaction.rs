macro_rules! deps {
    () => {
        TransactionDB!();
        OptimisticTransactionDB!();
        DB!();
    };
}

macro_rules! Transaction {
    () => {
        deps!();
        # [doc = " RocksDB Transaction."] # [doc = ""] # [doc = " To use transactions, you must first create a [`TransactionDB`] or [`OptimisticTransactionDB`]."] # [doc = ""] # [doc = " [`TransactionDB`]: crate::TransactionDB"] # [doc = " [`OptimisticTransactionDB`]: crate::OptimisticTransactionDB"] pub struct Transaction < 'db , DB > { pub (crate) inner : * mut ffi :: rocksdb_transaction_t , pub (crate) _marker : PhantomData < & 'db DB > , }
    };
}

Transaction!();
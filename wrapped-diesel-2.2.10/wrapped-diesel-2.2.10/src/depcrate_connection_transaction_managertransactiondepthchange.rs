// Generated macro for TransactionDepthChange (enum)
macro_rules! Depcrate_connection_transaction_managerTransactionDepthChange {
() => {
// Module: crate::connection::transaction_manager
// Provides: {"TransactionDepthChange"}
// Dependencies: {}
# [doc = " Represents a change to apply to the depth of a transaction"] # [derive (Debug , Clone , Copy)] pub enum TransactionDepthChange { # [doc = " Increase the depth of the transaction (corresponds to `BEGIN` or `SAVEPOINT`)"] IncreaseDepth , # [doc = " Decreases the depth of the transaction (corresponds to `COMMIT`/`RELEASE SAVEPOINT` or `ROLLBACK`)"] DecreaseDepth , }
};
}

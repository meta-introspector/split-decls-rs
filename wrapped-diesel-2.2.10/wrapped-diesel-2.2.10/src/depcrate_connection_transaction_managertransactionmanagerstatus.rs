// Generated macro for TransactionManagerStatus (enum)
macro_rules! Depcrate_connection_transaction_managerTransactionManagerStatus {
() => {
// Module: crate::connection::transaction_manager
// Provides: {"TransactionManagerStatus"}
// Dependencies: {}
# [doc = " Status of the transaction manager"] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")] # [derive (Debug)] pub enum TransactionManagerStatus { # [doc = " Valid status, the manager can run operations"] Valid (ValidTransactionManagerStatus) , # [doc = " Error status, probably following a broken connection. The manager will no longer run operations"] InError , }
};
}

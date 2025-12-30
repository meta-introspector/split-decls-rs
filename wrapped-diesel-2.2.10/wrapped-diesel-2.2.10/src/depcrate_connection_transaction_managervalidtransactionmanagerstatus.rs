// Generated macro for ValidTransactionManagerStatus (struct)
macro_rules! Depcrate_connection_transaction_managerValidTransactionManagerStatus {
() => {
// Module: crate::connection::transaction_manager
// Provides: {"ValidTransactionManagerStatus"}
// Dependencies: {}
# [doc = " Valid transaction status for the manager. Can return the current transaction depth"] # [allow (missing_copy_implementations)] # [derive (Debug , Default)] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , public_fields (in_transaction))] pub struct ValidTransactionManagerStatus { # [doc = " Inner status, or `None` if no transaction is running"] in_transaction : Option < InTransactionStatus > , }
};
}

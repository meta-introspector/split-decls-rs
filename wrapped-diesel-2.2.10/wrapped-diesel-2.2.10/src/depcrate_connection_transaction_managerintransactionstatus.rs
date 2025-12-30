// Generated macro for InTransactionStatus (struct)
macro_rules! Depcrate_connection_transaction_managerInTransactionStatus {
() => {
// Module: crate::connection::transaction_manager
// Provides: {"InTransactionStatus"}
// Dependencies: {}
# [doc = " Various status fields to track the status of"] # [doc = " a transaction manager with a started transaction"] # [allow (missing_copy_implementations)] # [derive (Debug)] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , public_fields (test_transaction , transaction_depth , requires_rollback_maybe_up_to_top_level))] pub struct InTransactionStatus { # [doc = " The current depth of nested transactions"] transaction_depth : NonZeroU32 , # [doc = " If that is registered, savepoints rollbacks will still be attempted, but failure to do so"] # [doc = " will not result in an error. (Some may succeed, some may not.)"] requires_rollback_maybe_up_to_top_level : bool , # [doc = " Is this transaction manager status marked as test-transaction?"] test_transaction : bool , }
};
}

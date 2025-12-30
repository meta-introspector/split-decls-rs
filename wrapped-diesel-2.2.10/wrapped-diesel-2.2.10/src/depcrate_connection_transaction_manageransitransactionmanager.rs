// Generated macro for AnsiTransactionManager (struct)
macro_rules! Depcrate_connection_transaction_managerAnsiTransactionManager {
() => {
// Module: crate::connection::transaction_manager
// Provides: {"AnsiTransactionManager"}
// Dependencies: {}
# [doc = " An implementation of `TransactionManager` which can be used for backends"] # [doc = " which use ANSI standard syntax for savepoints such as SQLite and PostgreSQL."] # [derive (Default , Debug)] pub struct AnsiTransactionManager { pub (crate) status : TransactionManagerStatus , }
};
}

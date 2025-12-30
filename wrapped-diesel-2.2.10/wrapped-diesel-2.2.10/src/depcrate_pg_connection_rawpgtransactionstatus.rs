// Generated macro for PgTransactionStatus (enum)
macro_rules! Depcrate_pg_connection_rawPgTransactionStatus {
() => {
// Module: crate::pg::connection::raw
// Provides: {"PgTransactionStatus"}
// Dependencies: {}
# [doc = " Represents the current in-transaction status of the connection"] # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub (super) enum PgTransactionStatus { # [doc = " Currently idle"] Idle , # [doc = " A command is in progress (sent to the server but not yet completed)"] Active , # [doc = " Idle, in a valid transaction block"] InTransaction , # [doc = " Idle, in a failed transaction block"] InError , # [doc = " Bad connection"] Unknown , }
};
}

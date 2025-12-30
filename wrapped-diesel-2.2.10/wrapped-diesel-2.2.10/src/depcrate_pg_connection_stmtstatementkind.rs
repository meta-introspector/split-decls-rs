// Generated macro for StatementKind (enum)
macro_rules! Depcrate_pg_connection_stmtStatementKind {
() => {
// Module: crate::pg::connection::stmt
// Provides: {"StatementKind"}
// Dependencies: {}
enum StatementKind { Unnamed { sql : CString , param_types : Vec < u32 > } , Named { name : CString } , }
};
}

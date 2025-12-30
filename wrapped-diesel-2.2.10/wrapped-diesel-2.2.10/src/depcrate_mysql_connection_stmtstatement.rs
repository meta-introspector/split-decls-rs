// Generated macro for Statement (struct)
macro_rules! Depcrate_mysql_connection_stmtStatement {
() => {
// Module: crate::mysql::connection::stmt
// Provides: {"Statement"}
// Dependencies: {}
# [allow (dead_code , missing_debug_implementations)] pub struct Statement { stmt : NonNull < ffi :: MYSQL_STMT > , input_binds : Option < PreparedStatementBinds > , }
};
}

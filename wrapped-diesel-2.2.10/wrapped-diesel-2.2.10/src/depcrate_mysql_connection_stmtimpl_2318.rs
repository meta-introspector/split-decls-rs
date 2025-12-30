// Generated macro for impl_2318 (impl)
macro_rules! Depcrate_mysql_connection_stmtimpl_2318 {
() => {
// Module: crate::mysql::connection::stmt
// Provides: {"impl_2318"}
// Dependencies: {}
impl Drop for StatementUse < '_ > { fn drop (& mut self) { unsafe { ffi :: mysql_stmt_free_result (self . inner . stmt . as_ptr ()) ; } } }
};
}

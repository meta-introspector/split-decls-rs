// Generated macro for impl_2315 (impl)
macro_rules! Depcrate_mysql_connection_stmtimpl_2315 {
() => {
// Module: crate::mysql::connection::stmt
// Provides: {"impl_2315"}
// Dependencies: {}
impl Drop for Statement { fn drop (& mut self) { unsafe { ffi :: mysql_stmt_close (self . stmt . as_ptr ()) } ; } }
};
}

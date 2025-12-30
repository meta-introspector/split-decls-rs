// Generated macro for impl_2308 (impl)
macro_rules! Depcrate_mysql_connection_stmt_metadataimpl_2308 {
() => {
// Module: crate::mysql::connection::stmt::metadata
// Provides: {"impl_2308"}
// Dependencies: {}
impl Drop for StatementMetadata { fn drop (& mut self) { unsafe { ffi :: mysql_free_result (self . result . as_mut ()) } ; } }
};
}

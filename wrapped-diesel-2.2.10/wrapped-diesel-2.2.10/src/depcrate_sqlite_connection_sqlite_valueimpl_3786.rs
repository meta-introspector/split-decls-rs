// Generated macro for impl_3786 (impl)
macro_rules! Depcrate_sqlite_connection_sqlite_valueimpl_3786 {
() => {
// Module: crate::sqlite::connection::sqlite_value
// Provides: {"impl_3786"}
// Dependencies: {}
impl Drop for OwnedSqliteValue { fn drop (& mut self) { unsafe { ffi :: sqlite3_value_free (self . value . as_ptr ()) } } }
};
}

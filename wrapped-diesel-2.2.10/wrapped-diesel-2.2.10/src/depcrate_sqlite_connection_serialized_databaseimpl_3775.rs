// Generated macro for impl_3775 (impl)
macro_rules! Depcrate_sqlite_connection_serialized_databaseimpl_3775 {
() => {
// Module: crate::sqlite::connection::serialized_database
// Provides: {"impl_3775"}
// Dependencies: {}
impl Drop for SerializedDatabase { # [doc = " Deallocates the memory of the serialized database when it goes out of scope."] fn drop (& mut self) { unsafe { ffi :: sqlite3_free (self . data as _) ; } } }
};
}

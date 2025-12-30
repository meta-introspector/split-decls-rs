// Generated macro for last_error (function)
macro_rules! Depcrate_sqlite_connection_stmtlast_error {
() => {
// Module: crate::sqlite::connection::stmt
// Provides: {"last_error"}
// Dependencies: {}
fn last_error (raw_connection : * mut ffi :: sqlite3) -> Error { let error_message = last_error_message (raw_connection) ; let error_information = Box :: new (error_message) ; let error_kind = match last_error_code (raw_connection) { ffi :: SQLITE_CONSTRAINT_UNIQUE | ffi :: SQLITE_CONSTRAINT_PRIMARYKEY => { DatabaseErrorKind :: UniqueViolation } ffi :: SQLITE_CONSTRAINT_FOREIGNKEY => DatabaseErrorKind :: ForeignKeyViolation , ffi :: SQLITE_CONSTRAINT_NOTNULL => DatabaseErrorKind :: NotNullViolation , ffi :: SQLITE_CONSTRAINT_CHECK => DatabaseErrorKind :: CheckViolation , _ => DatabaseErrorKind :: Unknown , } ; DatabaseError (error_kind , error_information) }
};
}

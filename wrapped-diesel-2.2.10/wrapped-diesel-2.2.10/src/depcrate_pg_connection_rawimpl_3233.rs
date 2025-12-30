// Generated macro for impl_3233 (impl)
macro_rules! Depcrate_pg_connection_rawimpl_3233 {
() => {
// Module: crate::pg::connection::raw
// Provides: {"impl_3233"}
// Dependencies: {}
impl RawResult { # [allow (clippy :: new_ret_no_self)] fn new (ptr : * mut PGresult , conn : & RawConnection) -> QueryResult < Self > { NonNull :: new (ptr) . map (RawResult) . ok_or_else (| | { Error :: DatabaseError (DatabaseErrorKind :: UnableToSendCommand , Box :: new (conn . last_error_message ()) ,) }) } pub (super) fn as_ptr (& self) -> * mut PGresult { self . 0 . as_ptr () } pub (super) fn error_message (& self) -> & str { let ptr = unsafe { PQresultErrorMessage (self . 0 . as_ptr ()) } ; let cstr = unsafe { CStr :: from_ptr (ptr) } ; cstr . to_str () . unwrap_or_default () } }
};
}

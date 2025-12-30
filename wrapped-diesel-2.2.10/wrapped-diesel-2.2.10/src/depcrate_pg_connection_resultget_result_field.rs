// Generated macro for get_result_field (function)
macro_rules! Depcrate_pg_connection_resultget_result_field {
() => {
// Module: crate::pg::connection::result
// Provides: {"get_result_field"}
// Dependencies: {}
fn get_result_field < 'a > (res : * mut PGresult , field : ResultField) -> Option < & 'a str > { let ptr = unsafe { PQresultErrorField (res , field as libc :: c_int) } ; if ptr . is_null () { return None ; } let c_str = unsafe { CStr :: from_ptr (ptr) } ; c_str . to_str () . ok () }
};
}

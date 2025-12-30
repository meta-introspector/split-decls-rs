// Generated macro for impl_3734 (impl)
macro_rules! Depcrate_sqlite_connection_rawimpl_3734 {
() => {
// Module: crate::sqlite::connection::raw
// Provides: {"impl_3734"}
// Dependencies: {}
impl Drop for RawConnection { fn drop (& mut self) { use std :: thread :: panicking ; let close_result = unsafe { ffi :: sqlite3_close (self . internal_connection . as_ptr ()) } ; if close_result != ffi :: SQLITE_OK { let error_message = super :: error_message (close_result) ; if panicking () { write ! (stderr () , "Error closing SQLite connection: {error_message}") . expect ("Error writing to `stderr`") ; } else { panic ! ("Error closing SQLite connection: {}" , error_message) ; } } } }
};
}

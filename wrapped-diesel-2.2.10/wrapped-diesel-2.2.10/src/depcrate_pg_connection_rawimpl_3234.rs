// Generated macro for impl_3234 (impl)
macro_rules! Depcrate_pg_connection_rawimpl_3234 {
() => {
// Module: crate::pg::connection::raw
// Provides: {"impl_3234"}
// Dependencies: {}
impl Drop for RawResult { fn drop (& mut self) { unsafe { PQclear (self . 0 . as_ptr ()) } } }
};
}

// Generated macro for impl_3228 (impl)
macro_rules! Depcrate_pg_connection_rawimpl_3228 {
() => {
// Module: crate::pg::connection::raw
// Provides: {"impl_3228"}
// Dependencies: {}
impl Drop for RawConnection { fn drop (& mut self) { unsafe { PQfinish (self . internal_connection . as_ptr ()) } ; } }
};
}

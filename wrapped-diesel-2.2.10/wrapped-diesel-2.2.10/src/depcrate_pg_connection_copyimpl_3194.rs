// Generated macro for impl_3194 (impl)
macro_rules! Depcrate_pg_connection_copyimpl_3194 {
() => {
// Module: crate::pg::connection::copy
// Provides: {"impl_3194"}
// Dependencies: {}
impl Drop for CopyToBuffer < '_ > { # [allow (unsafe_code)] fn drop (& mut self) { if ! self . ptr . is_null () { unsafe { pq_sys :: PQfreemem (self . ptr as * mut ffi :: c_void) } ; self . ptr = std :: ptr :: null_mut () ; } } }
};
}

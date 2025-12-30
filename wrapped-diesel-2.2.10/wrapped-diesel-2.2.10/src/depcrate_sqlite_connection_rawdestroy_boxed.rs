// Generated macro for destroy_boxed (function)
macro_rules! Depcrate_sqlite_connection_rawdestroy_boxed {
() => {
// Module: crate::sqlite::connection::raw
// Provides: {"destroy_boxed"}
// Dependencies: {}
extern "C" fn destroy_boxed < F > (data : * mut libc :: c_void) { let ptr = data as * mut F ; unsafe { std :: mem :: drop (Box :: from_raw (ptr)) } ; }
};
}

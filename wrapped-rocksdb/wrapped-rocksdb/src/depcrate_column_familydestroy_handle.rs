// Generated macro for destroy_handle (function)
macro_rules! Depcrate_column_familydestroy_handle {
() => {
// Module: crate::column_family
// Provides: {"destroy_handle"}
// Dependencies: {}
fn destroy_handle (handle : * mut ffi :: rocksdb_column_family_handle_t) { unsafe { ffi :: rocksdb_column_family_handle_destroy (handle) ; } }
};
}

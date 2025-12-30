// Generated macro for free_demangled_name (function)
macro_rules! Depcrate_c_apifree_demangled_name {
() => {
// Module: crate::c_api
// Provides: {"free_demangled_name"}
// Dependencies: {}
# [no_mangle] pub unsafe extern "C" fn free_demangled_name (buffer : * mut raw :: c_char) { if buffer . is_null () { return ; } ffi :: CString :: from_raw (buffer) ; }
};
}

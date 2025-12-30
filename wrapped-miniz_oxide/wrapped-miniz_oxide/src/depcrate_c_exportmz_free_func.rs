// Generated macro for mz_free_func (type)
macro_rules! Depcrate_c_exportmz_free_func {
() => {
// Module: crate::c_export
// Provides: {"mz_free_func"}
// Dependencies: {}
# [doc = " Signature of function used to free the compressor/decompressor structs."] # [allow (bad_style)] pub type mz_free_func = unsafe extern "C" fn (* mut c_void , * mut c_void) ;
};
}

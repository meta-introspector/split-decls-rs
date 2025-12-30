// Generated macro for mz_alloc_func (type)
macro_rules! Depcrate_c_exportmz_alloc_func {
() => {
// Module: crate::c_export
// Provides: {"mz_alloc_func"}
// Dependencies: {}
# [doc = " Signature of function used to allocate the compressor/decompressor structs."] # [allow (bad_style)] pub type mz_alloc_func = unsafe extern "C" fn (* mut c_void , size_t , size_t) -> * mut c_void ;
};
}

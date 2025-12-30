// Generated macro for mz_alloc_callback (type)
macro_rules! Depcrate_c_exportmz_alloc_callback {
() => {
// Module: crate::c_export
// Provides: {"mz_alloc_callback"}
// Dependencies: {}
# [allow (bad_style)] pub type mz_alloc_callback = Option < unsafe extern "C" fn (* mut c_void , size_t , size_t) -> * mut c_void > ;
};
}

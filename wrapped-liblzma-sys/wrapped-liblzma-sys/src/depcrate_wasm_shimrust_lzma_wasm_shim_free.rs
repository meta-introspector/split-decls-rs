// Generated macro for rust_lzma_wasm_shim_free (function)
macro_rules! Depcrate_wasm_shimrust_lzma_wasm_shim_free {
() => {
// Module: crate::wasm_shim
// Provides: {"rust_lzma_wasm_shim_free"}
// Dependencies: {}
# [no_mangle] pub unsafe extern "C" fn rust_lzma_wasm_shim_free (ptr : * mut c_void) { if ptr . is_null () { return ; } wasm_shim_free (ptr) }
};
}

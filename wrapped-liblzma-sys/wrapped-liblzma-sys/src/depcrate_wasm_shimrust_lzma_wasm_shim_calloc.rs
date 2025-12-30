// Generated macro for rust_lzma_wasm_shim_calloc (function)
macro_rules! Depcrate_wasm_shimrust_lzma_wasm_shim_calloc {
() => {
// Module: crate::wasm_shim
// Provides: {"rust_lzma_wasm_shim_calloc"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn rust_lzma_wasm_shim_calloc (nmemb : usize , size : usize) -> * mut c_void { wasm_shim_alloc :: < true > (nmemb * size) }
};
}

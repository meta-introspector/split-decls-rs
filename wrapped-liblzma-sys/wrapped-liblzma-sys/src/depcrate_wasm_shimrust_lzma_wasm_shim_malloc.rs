// Generated macro for rust_lzma_wasm_shim_malloc (function)
macro_rules! Depcrate_wasm_shimrust_lzma_wasm_shim_malloc {
() => {
// Module: crate::wasm_shim
// Provides: {"rust_lzma_wasm_shim_malloc"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn rust_lzma_wasm_shim_malloc (size : usize) -> * mut c_void { wasm_shim_alloc :: < false > (size) }
};
}

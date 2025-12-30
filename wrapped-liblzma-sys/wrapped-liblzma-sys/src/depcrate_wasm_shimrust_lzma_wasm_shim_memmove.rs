// Generated macro for rust_lzma_wasm_shim_memmove (function)
macro_rules! Depcrate_wasm_shimrust_lzma_wasm_shim_memmove {
() => {
// Module: crate::wasm_shim
// Provides: {"rust_lzma_wasm_shim_memmove"}
// Dependencies: {}
# [no_mangle] pub unsafe extern "C" fn rust_lzma_wasm_shim_memmove (dest : * mut c_void , src : * const c_void , n : usize ,) -> * mut c_void { core :: ptr :: copy (src as * const u8 , dest as * mut u8 , n) ; dest }
};
}

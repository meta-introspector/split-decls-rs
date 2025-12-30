// Generated macro for rust_lzma_wasm_shim_strlen (function)
macro_rules! Depcrate_wasm_shimrust_lzma_wasm_shim_strlen {
() => {
// Module: crate::wasm_shim
// Provides: {"rust_lzma_wasm_shim_strlen"}
// Dependencies: {}
# [no_mangle] pub unsafe extern "C" fn rust_lzma_wasm_shim_strlen (s : * const c_char) -> usize { let str = unsafe { std :: ffi :: CStr :: from_ptr (s) } ; str . to_bytes () . len () }
};
}

// Generated macro for wasm_shim_free (function)
macro_rules! Depcrate_wasm_shimwasm_shim_free {
() => {
// Module: crate::wasm_shim
// Provides: {"wasm_shim_free"}
// Dependencies: {}
unsafe fn wasm_shim_free (ptr : * mut c_void) { let alloc_ptr = ptr . sub (USIZE_SIZE) ; let full_alloc_size = alloc_ptr . cast :: < usize > () . read () ; let layout = Layout :: from_size_align_unchecked (full_alloc_size , USIZE_ALIGN) ; dealloc (alloc_ptr . cast () , layout) ; }
};
}

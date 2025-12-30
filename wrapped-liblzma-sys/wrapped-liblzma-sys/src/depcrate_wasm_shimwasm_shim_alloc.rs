// Generated macro for wasm_shim_alloc (function)
macro_rules! Depcrate_wasm_shimwasm_shim_alloc {
() => {
// Module: crate::wasm_shim
// Provides: {"wasm_shim_alloc"}
// Dependencies: {}
# [inline] fn wasm_shim_alloc < const ZEROED : bool > (size : usize) -> * mut c_void { let full_alloc_size = size + USIZE_SIZE ; unsafe { let layout = Layout :: from_size_align_unchecked (full_alloc_size , USIZE_ALIGN) ; let ptr = if ZEROED { alloc_zeroed (layout) } else { alloc (layout) } ; ptr . cast :: < usize > () . write (full_alloc_size) ; ptr . add (USIZE_SIZE) . cast () } }
};
}

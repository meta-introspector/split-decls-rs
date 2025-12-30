// Generated macro for other_45 (other)
macro_rules! Depcrate_allocother_45 {
() => {
// Module: crate::alloc
// Provides: {"other_45"}
// Dependencies: {}
unsafe extern "Rust" { # [rustc_allocator] # [rustc_nounwind] # [rustc_std_internal_symbol] fn __rust_alloc (size : usize , align : usize) -> * mut u8 ; # [rustc_deallocator] # [rustc_nounwind] # [rustc_std_internal_symbol] fn __rust_dealloc (ptr : * mut u8 , size : usize , align : usize) ; # [rustc_reallocator] # [rustc_nounwind] # [rustc_std_internal_symbol] fn __rust_realloc (ptr : * mut u8 , old_size : usize , align : usize , new_size : usize) -> * mut u8 ; # [rustc_allocator_zeroed] # [rustc_nounwind] # [rustc_std_internal_symbol] fn __rust_alloc_zeroed (size : usize , align : usize) -> * mut u8 ; # [rustc_nounwind] # [rustc_std_internal_symbol] fn __rust_no_alloc_shim_is_unstable_v2 () ; }
};
}

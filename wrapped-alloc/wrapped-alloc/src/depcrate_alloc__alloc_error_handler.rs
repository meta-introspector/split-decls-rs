// Generated macro for __alloc_error_handler (module)
macro_rules! Depcrate_alloc__alloc_error_handler {
() => {
// Module: crate::alloc
// Provides: {"__alloc_error_handler"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [doc (hidden)] # [allow (unused_attributes)] # [unstable (feature = "alloc_internals" , issue = "none")] pub mod __alloc_error_handler { # [rustc_std_internal_symbol] pub unsafe fn __rdl_oom (size : usize , _align : usize) -> ! { unsafe extern "Rust" { # [rustc_std_internal_symbol] fn __rust_alloc_error_handler_should_panic_v2 () -> u8 ; } if unsafe { __rust_alloc_error_handler_should_panic_v2 () != 0 } { panic ! ("memory allocation of {size} bytes failed") } else { core :: panicking :: panic_nounwind_fmt (format_args ! ("memory allocation of {size} bytes failed") , false ,) } } }
};
}

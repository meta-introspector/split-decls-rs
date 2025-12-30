// Generated macro for impl_83 (impl)
macro_rules! Depcrate_generatedimpl_83 {
() => {
// Module: crate::generated
// Provides: {"impl_83"}
// Dependencies: {}
impl DispatchOnce { # [doc = " # Safety"] # [doc = ""] # [doc = " - `predicate` must be a valid pointer."] # [doc = " - `block` must be a valid pointer."] # [doc (alias = "dispatch_once")] # [cfg (feature = "block2")] # [inline] pub unsafe fn once_with_block (predicate : NonNull < dispatch_once_t > , block : dispatch_block_t) { extern "C" { fn dispatch_once (predicate : NonNull < dispatch_once_t > , block : dispatch_block_t) ; } unsafe { dispatch_once (predicate , block) } } # [doc = " # Safety"] # [doc = ""] # [doc = " - `predicate` must be a valid pointer."] # [doc = " - `context` must be a valid pointer or null."] # [doc = " - `function` must be implemented correctly."] # [doc (alias = "dispatch_once_f")] # [inline] pub unsafe fn once_f (predicate : NonNull < dispatch_once_t > , context : * mut c_void , function : dispatch_function_t ,) { extern "C" { fn dispatch_once_f (predicate : NonNull < dispatch_once_t > , context : * mut c_void , function : dispatch_function_t ,) ; } unsafe { dispatch_once_f (predicate , context , function) } } }
};
}

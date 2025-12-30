// Generated macro for macro_9819 (macro)
macro_rules! Depcrate_swap_ptr_to_refmacro_9819 {
() => {
// Module: crate::swap_ptr_to_ref
// Provides: {"macro_9819"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `core::mem::swap` where either parameter is derived from a pointer"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " When at least one parameter to `swap` is derived from a pointer it may overlap with the"] # [doc = " other. This would then lead to undefined behavior."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " unsafe fn swap(x: &[*mut u32], y: &[*mut u32]) {"] # [doc = "     for (&x, &y) in x.iter().zip(y) {"] # [doc = "         core::mem::swap(&mut *x, &mut *y);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " unsafe fn swap(x: &[*mut u32], y: &[*mut u32]) {"] # [doc = "     for (&x, &y) in x.iter().zip(y) {"] # [doc = "         core::ptr::swap(x, y);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.63.0"] pub SWAP_PTR_TO_REF , suspicious , "call to `mem::swap` using pointer derived references" }
};
}

// Generated macro for macro_9064 (macro)
macro_rules! Depcrate_ptrmacro_9064 {
() => {
// Module: crate::ptr
// Provides: {"macro_9064"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Use `std::ptr::eq` when applicable"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `ptr::eq` can be used to compare `&T` references"] # [doc = " (which coerce to `*const T` implicitly) by their address rather than"] # [doc = " comparing the values they point to."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let a = &[1, 2, 3];"] # [doc = " let b = &[1, 2, 3];"] # [doc = ""] # [doc = " assert!(a as *const _ as usize == b as *const _ as usize);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let a = &[1, 2, 3];"] # [doc = " let b = &[1, 2, 3];"] # [doc = ""] # [doc = " assert!(std::ptr::eq(a, b));"] # [doc = " ```"] # [clippy :: version = "1.49.0"] pub PTR_EQ , style , "use `std::ptr::eq` when comparing raw pointers" }
};
}

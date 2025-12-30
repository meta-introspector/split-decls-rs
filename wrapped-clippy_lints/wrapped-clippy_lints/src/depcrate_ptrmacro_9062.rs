// Generated macro for macro_9062 (macro)
macro_rules! Depcrate_ptrmacro_9062 {
() => {
// Module: crate::ptr
// Provides: {"macro_9062"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " This lint checks for equality comparisons with `ptr::null`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's easier and more readable to use the inherent"] # [doc = " `.is_null()`"] # [doc = " method instead"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " use std::ptr;"] # [doc = ""] # [doc = " if x == ptr::null {"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " if x.is_null() {"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub CMP_NULL , style , "comparing a pointer to a null pointer, suggesting to use `.is_null()` instead" }
};
}

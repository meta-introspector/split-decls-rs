// Generated macro for macro_7239 (macro)
macro_rules! Depcrate_methodsmacro_7239 {
() => {
// Module: crate::methods
// Provides: {"macro_7239"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks the usage of `.first().is_some()` or `.first().is_none()` to check if a slice is"] # [doc = " empty."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `.is_empty()` is shorter and better communicates the intention."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let v = vec![1, 2, 3];"] # [doc = " if v.first().is_none() {"] # [doc = "     // The vector is empty..."] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let v = vec![1, 2, 3];"] # [doc = " if v.is_empty() {"] # [doc = "     // The vector is empty..."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.83.0"] pub UNNECESSARY_FIRST_THEN_CHECK , complexity , "calling `.first().is_some()` or `.first().is_none()` instead of `.is_empty()`" }
};
}

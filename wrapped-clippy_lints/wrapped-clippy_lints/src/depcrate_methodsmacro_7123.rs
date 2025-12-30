// Generated macro for macro_7123 (macro)
macro_rules! Depcrate_methodsmacro_7123 {
() => {
// Module: crate::methods
// Provides: {"macro_7123"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `_.filter(_).map(_)` that can be written more simply"] # [doc = " as `filter_map(_)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Redundant code in the `filter` and `map` operations is poor style and"] # [doc = " less performant."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " (0_i32..10)"] # [doc = "     .filter(|n| n.checked_add(1).is_some())"] # [doc = "     .map(|n| n.checked_add(1).unwrap());"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " (0_i32..10).filter_map(|n| n.checked_add(1));"] # [doc = " ```"] # [clippy :: version = "1.51.0"] pub MANUAL_FILTER_MAP , complexity , "using `_.filter(_).map(_)` in a way that can be written more simply as `filter_map(_)`" }
};
}

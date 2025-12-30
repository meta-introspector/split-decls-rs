// Generated macro for macro_6977 (macro)
macro_rules! Depcrate_methodsmacro_6977 {
() => {
// Module: crate::methods
// Provides: {"macro_6977"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `_.find(_).map(_)` that can be written more simply"] # [doc = " as `find_map(_)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Redundant code in the `find` and `map` operations is poor style and"] # [doc = " less performant."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " (0_i32..10)"] # [doc = "     .find(|n| n.checked_add(1).is_some())"] # [doc = "     .map(|n| n.checked_add(1).unwrap());"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " (0_i32..10).find_map(|n| n.checked_add(1));"] # [doc = " ```"] # [clippy :: version = "1.51.0"] pub MANUAL_FIND_MAP , complexity , "using `_.find(_).map(_)` in a way that can be written more simply as `find_map(_)`" }
};
}

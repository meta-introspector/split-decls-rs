// Generated macro for macro_1854 (macro)
macro_rules! Depcrate_empty_line_aftermacro_1854 {
() => {
// Module: crate::empty_line_after
// Provides: {"macro_1854"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for empty lines after outer attributes"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The attribute may have meant to be an inner attribute (`#![attr]`). If"] # [doc = " it was meant to be an outer attribute (`#[attr]`) then the empty line"] # [doc = " should be removed"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #[allow(dead_code)]"] # [doc = ""] # [doc = " fn not_quite_good_code() {}"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " // Good (as inner attribute)"] # [doc = " #![allow(dead_code)]"] # [doc = ""] # [doc = " fn this_is_fine() {}"] # [doc = ""] # [doc = " // or"] # [doc = ""] # [doc = " // Good (as outer attribute)"] # [doc = " #[allow(dead_code)]"] # [doc = " fn this_is_fine_too() {}"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub EMPTY_LINE_AFTER_OUTER_ATTR , suspicious , "empty line after outer attribute" }
};
}

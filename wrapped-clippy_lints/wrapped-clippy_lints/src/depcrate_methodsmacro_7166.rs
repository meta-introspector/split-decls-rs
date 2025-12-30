// Generated macro for macro_7166 (macro)
macro_rules! Depcrate_methodsmacro_7166 {
() => {
// Module: crate::methods
// Provides: {"macro_7166"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `filter_map(|x| x)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability, this can be written more concisely by using `flatten`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let iter = vec![Some(1)].into_iter();"] # [doc = " iter.filter_map(|x| x);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let iter = vec![Some(1)].into_iter();"] # [doc = " iter.flatten();"] # [doc = " ```"] # [clippy :: version = "1.52.0"] pub FILTER_MAP_IDENTITY , complexity , "call to `filter_map` where `flatten` is sufficient" }
};
}

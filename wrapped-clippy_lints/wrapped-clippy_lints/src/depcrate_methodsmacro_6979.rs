// Generated macro for macro_6979 (macro)
macro_rules! Depcrate_methodsmacro_6979 {
() => {
// Module: crate::methods
// Provides: {"macro_6979"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `flat_map(|x| x)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability, this can be written more concisely by using `flatten`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let iter = vec![vec![0]].into_iter();"] # [doc = " iter.flat_map(|x| x);"] # [doc = " ```"] # [doc = " Can be written as"] # [doc = " ```no_run"] # [doc = " # let iter = vec![vec![0]].into_iter();"] # [doc = " iter.flatten();"] # [doc = " ```"] # [clippy :: version = "1.39.0"] pub FLAT_MAP_IDENTITY , complexity , "call to `flat_map` where `flatten` is sufficient" }
};
}

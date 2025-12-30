// Generated macro for macro_6990 (macro)
macro_rules! Depcrate_methodsmacro_6990 {
() => {
// Module: crate::methods
// Provides: {"macro_6990"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for iterators of `Option`s using `.filter(Option::is_some).map(Option::unwrap)` that may"] # [doc = " be replaced with a `.flatten()` call."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `Option` is like a collection of 0-1 things, so `flatten`"] # [doc = " automatically does this without suspicious-looking `unwrap` calls."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _ = std::iter::empty::<Option<i32>>().filter(Option::is_some).map(Option::unwrap);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let _ = std::iter::empty::<Option<i32>>().flatten();"] # [doc = " ```"] # [clippy :: version = "1.53.0"] pub OPTION_FILTER_MAP , complexity , "filtering `Option` for `Some` then force-unwrapping, which can be one type-safe operation" }
};
}

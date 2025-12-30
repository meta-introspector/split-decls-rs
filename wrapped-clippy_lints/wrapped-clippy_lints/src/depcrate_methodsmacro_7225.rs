// Generated macro for macro_7225 (macro)
macro_rules! Depcrate_methodsmacro_7225 {
() => {
// Module: crate::methods
// Provides: {"macro_7225"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for iterators of `Result`s using `.filter(Result::is_ok).map(Result::unwrap)` that may"] # [doc = " be replaced with a `.flatten()` call."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `Result` implements `IntoIterator<Item = T>`. This means that `Result` can be flattened"] # [doc = " automatically without suspicious-looking `unwrap` calls."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _ = std::iter::empty::<Result<i32, ()>>().filter(Result::is_ok).map(Result::unwrap);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let _ = std::iter::empty::<Result<i32, ()>>().flatten();"] # [doc = " ```"] # [clippy :: version = "1.77.0"] pub RESULT_FILTER_MAP , complexity , "filtering `Result` for `Ok` then force-unwrapping, which can be one type-safe operation" }
};
}

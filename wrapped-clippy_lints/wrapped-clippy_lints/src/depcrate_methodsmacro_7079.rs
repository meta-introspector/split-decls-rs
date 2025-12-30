// Generated macro for macro_7079 (macro)
macro_rules! Depcrate_methodsmacro_7079 {
() => {
// Module: crate::methods
// Provides: {"macro_7079"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.filter(Result::is_ok)` that may be replaced with a `.flatten()` call."] # [doc = " This lint will require additional changes to the follow-up calls as it affects the type."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This pattern is often followed by manual unwrapping of `Result`. The simplification"] # [doc = " results in more readable and succinct code without the need for manual unwrapping."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " vec![Ok::<i32, String>(1)].into_iter().filter(Result::is_ok);"] # [doc = ""] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " vec![Ok::<i32, String>(1)].into_iter().flatten();"] # [doc = " ```"] # [clippy :: version = "1.77.0"] pub ITER_FILTER_IS_OK , pedantic , "filtering an iterator over `Result`s for `Ok` can be achieved with `flatten`" }
};
}

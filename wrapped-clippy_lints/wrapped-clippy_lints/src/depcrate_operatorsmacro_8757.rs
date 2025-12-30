// Generated macro for macro_8757 (macro)
macro_rules! Depcrate_operatorsmacro_8757 {
() => {
// Module: crate::operators
// Provides: {"macro_8757"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for manual implementation of `.is_multiple_of()` on"] # [doc = " unsigned integer types."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `a.is_multiple_of(b)` is a clearer way to check for divisibility"] # [doc = " of `a` by `b`. This expression can never panic."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let (a, b) = (3u64, 4u64);"] # [doc = " if a % b == 0 {"] # [doc = "     println!(\"{a} is divisible by {b}\");"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let (a, b) = (3u64, 4u64);"] # [doc = " if a.is_multiple_of(b) {"] # [doc = "     println!(\"{a} is divisible by {b}\");"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.90.0"] pub MANUAL_IS_MULTIPLE_OF , complexity , "manual implementation of `.is_multiple_of()`" }
};
}

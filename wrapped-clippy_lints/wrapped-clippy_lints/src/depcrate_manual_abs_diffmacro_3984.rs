// Generated macro for macro_3984 (macro)
macro_rules! Depcrate_manual_abs_diffmacro_3984 {
() => {
// Module: crate::manual_abs_diff
// Provides: {"macro_3984"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects patterns like `if a > b { a - b } else { b - a }` and suggests using `a.abs_diff(b)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `abs_diff` is shorter, more readable, and avoids control flow."] # [doc = ""] # [doc = " ### Examples"] # [doc = " ```no_run"] # [doc = " # let (a, b) = (5_usize, 3_usize);"] # [doc = " if a > b {"] # [doc = "     a - b"] # [doc = " } else {"] # [doc = "     b - a"] # [doc = " }"] # [doc = " # ;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let (a, b) = (5_usize, 3_usize);"] # [doc = " a.abs_diff(b)"] # [doc = " # ;"] # [doc = " ```"] # [clippy :: version = "1.88.0"] pub MANUAL_ABS_DIFF , complexity , "using an if-else pattern instead of `abs_diff`" }
};
}

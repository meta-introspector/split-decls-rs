// Generated macro for macro_7022 (macro)
macro_rules! Depcrate_methodsmacro_7022 {
() => {
// Module: crate::methods
// Provides: {"macro_7022"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the use of `.iter().count()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `.len()` is more efficient and more"] # [doc = " readable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let some_vec = vec![0, 1, 2, 3];"] # [doc = ""] # [doc = " some_vec.iter().count();"] # [doc = " &some_vec[..].iter().count();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let some_vec = vec![0, 1, 2, 3];"] # [doc = ""] # [doc = " some_vec.len();"] # [doc = " &some_vec[..].len();"] # [doc = " ```"] # [clippy :: version = "1.52.0"] pub ITER_COUNT , complexity , "replace `.iter().count()` with `.len()`" }
};
}

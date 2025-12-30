// Generated macro for macro_2913 (macro)
macro_rules! Depcrate_infinite_itermacro_2913 {
() => {
// Module: crate::infinite_iter
// Provides: {"macro_2913"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for iteration that is guaranteed to be infinite."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " While there may be places where this is acceptable"] # [doc = " (e.g., in event streams), in most cases this is simply an error."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::iter;"] # [doc = ""] # [doc = " iter::repeat(1_u8).collect::<Vec<_>>();"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub INFINITE_ITER , correctness , "infinite iteration" }
};
}

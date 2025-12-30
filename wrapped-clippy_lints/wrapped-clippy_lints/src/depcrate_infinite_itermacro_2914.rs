// Generated macro for macro_2914 (macro)
macro_rules! Depcrate_infinite_itermacro_2914 {
() => {
// Module: crate::infinite_iter
// Provides: {"macro_2914"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for iteration that may be infinite."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " While there may be places where this is acceptable"] # [doc = " (e.g., in event streams), in most cases this is simply an error."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " The code may have a condition to stop iteration, but"] # [doc = " this lint is not clever enough to analyze it."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let infinite_iter = 0..;"] # [doc = " [0..].iter().zip(infinite_iter.take_while(|x| *x > 5));"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MAYBE_INFINITE_ITER , pedantic , "possible infinite iteration" }
};
}

// Generated macro for macro_7064 (macro)
macro_rules! Depcrate_methodsmacro_7064 {
() => {
// Module: crate::methods
// Provides: {"macro_7064"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `Iterator::fold` with a type that implements `Try`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The code should use `try_fold` instead, which short-circuits on failure, thus opening the"] # [doc = " door for additional optimizations not possible with `fold` as rustc can guarantee the"] # [doc = " function is never called on `None`, `Err`, etc., alleviating otherwise necessary checks. It's"] # [doc = " also slightly more idiomatic."] # [doc = ""] # [doc = " ### Known issues"] # [doc = " This lint doesn't take into account whether a function does something on the failure case,"] # [doc = " i.e., whether short-circuiting will affect behavior. Refactoring to `try_fold` is not"] # [doc = " desirable in those cases."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " vec![1, 2, 3].iter().fold(Some(0i32), |sum, i| sum?.checked_add(*i));"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " vec![1, 2, 3].iter().try_fold(0i32, |sum, i| sum.checked_add(*i));"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub MANUAL_TRY_FOLD , perf , "checks for usage of `Iterator::fold` with a type that implements `Try`" }
};
}

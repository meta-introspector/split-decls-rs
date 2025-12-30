// Generated macro for macro_6997 (macro)
macro_rules! Depcrate_methodsmacro_6997 {
() => {
// Module: crate::methods
// Provides: {"macro_6997"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for occurrences where one vector gets extended instead of append"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `append` instead of `extend` is more concise and faster"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut a = vec![1, 2, 3];"] # [doc = " let mut b = vec![4, 5, 6];"] # [doc = ""] # [doc = " a.extend(b.drain(..));"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let mut a = vec![1, 2, 3];"] # [doc = " let mut b = vec![4, 5, 6];"] # [doc = ""] # [doc = " a.append(&mut b);"] # [doc = " ```"] # [clippy :: version = "1.55.0"] pub EXTEND_WITH_DRAIN , perf , "using vec.append(&mut vec) to move the full range of a vector to another" }
};
}

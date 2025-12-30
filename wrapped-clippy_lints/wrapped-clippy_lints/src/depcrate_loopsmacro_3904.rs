// Generated macro for macro_3904 (macro)
macro_rules! Depcrate_loopsmacro_3904 {
() => {
// Module: crate::loops
// Provides: {"macro_3904"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for manual implementations of Iterator::find"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It doesn't affect performance, but using `find` is shorter and easier to read."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " fn example(arr: Vec<i32>) -> Option<i32> {"] # [doc = "     for el in arr {"] # [doc = "         if el == 1 {"] # [doc = "             return Some(el);"] # [doc = "         }"] # [doc = "     }"] # [doc = "     None"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn example(arr: Vec<i32>) -> Option<i32> {"] # [doc = "     arr.into_iter().find(|&el| el == 1)"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.64.0"] pub MANUAL_FIND , complexity , "manual implementation of `Iterator::find`" }
};
}

// Generated macro for macro_7048 (macro)
macro_rules! Depcrate_methodsmacro_7048 {
() => {
// Module: crate::methods
// Provides: {"macro_7048"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for zipping a collection with the range of"] # [doc = " `0.._.len()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The code is better expressed with `.enumerate()`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let x = vec![1];"] # [doc = " let _ = x.iter().zip(0..x.len());"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let x = vec![1];"] # [doc = " let _ = x.iter().enumerate();"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub RANGE_ZIP_WITH_LEN , complexity , "zipping iterator with a range when `enumerate()` would do" }
};
}

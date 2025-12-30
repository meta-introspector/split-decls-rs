// Generated macro for macro_3908 (macro)
macro_rules! Depcrate_loopsmacro_3908 {
() => {
// Module: crate::loops
// Provides: {"macro_3908"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for manually filling a slice with a value."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using the `fill` method is more idiomatic and concise."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut some_slice = [1, 2, 3, 4, 5];"] # [doc = " for i in 0..some_slice.len() {"] # [doc = "     some_slice[i] = 0;"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let mut some_slice = [1, 2, 3, 4, 5];"] # [doc = " some_slice.fill(0);"] # [doc = " ```"] # [clippy :: version = "1.86.0"] pub MANUAL_SLICE_FILL , style , "manually filling a slice with a value" }
};
}

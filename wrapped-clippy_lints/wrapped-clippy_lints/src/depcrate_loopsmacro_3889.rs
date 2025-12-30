// Generated macro for macro_3889 (macro)
macro_rules! Depcrate_loopsmacro_3889 {
() => {
// Module: crate::loops
// Provides: {"macro_3889"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for loops on `y.into_iter()` where `y` will do, and"] # [doc = " suggests the latter."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let y = vec![1];"] # [doc = " // with `y` a `Vec` or slice:"] # [doc = " for x in y.into_iter() {"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] # [doc = " can be rewritten to"] # [doc = " ```no_run"] # [doc = " # let y = vec![1];"] # [doc = " for x in y {"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub EXPLICIT_INTO_ITER_LOOP , pedantic , "for-looping over `_.into_iter()` when `_` would do" }
};
}

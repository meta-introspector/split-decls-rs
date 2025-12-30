// Generated macro for macro_3943 (macro)
macro_rules! Depcrate_loopsmacro_3943 {
() => {
// Module: crate::loops
// Provides: {"macro_3943"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for loops on `x.iter()` where `&x` will do, and"] # [doc = " suggests the latter."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " False negatives. We currently only warn on some known"] # [doc = " types."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " // with `y` a `Vec` or slice:"] # [doc = " # let y = vec![1];"] # [doc = " for x in y.iter() {"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let y = vec![1];"] # [doc = " for x in &y {"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub EXPLICIT_ITER_LOOP , pedantic , "for-looping over `_.iter()` or `_.iter_mut()` when `&_` or `&mut _` would do" }
};
}

// Generated macro for macro_1080 (macro)
macro_rules! Depcrate_castsmacro_1080 {
() => {
// Module: crate::casts
// Provides: {"macro_1080"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for casts from an unsigned type to a signed type of"] # [doc = " the same size, or possibly smaller due to target-dependent integers."] # [doc = " Performing such a cast is a no-op for the compiler (that is, nothing is"] # [doc = " changed at the bit level), and the binary representation of the value is"] # [doc = " reinterpreted. This can cause wrapping if the value is too big"] # [doc = " for the target signed type. However, the cast works as defined, so this lint"] # [doc = " is `Allow` by default."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " While such a cast is not bad in itself, the results can"] # [doc = " be surprising when this is not the intended behavior:"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _ = u32::MAX as i32; // will yield a value of `-1`"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let _ = i32::try_from(u32::MAX).ok();"] # [doc = " ```"] # [doc = ""] # [clippy :: version = "pre 1.29.0"] pub CAST_POSSIBLE_WRAP , pedantic , "casts that may cause wrapping around the value, e.g., `x as i32` where `x: u32` and `x > i32::MAX`" }
};
}

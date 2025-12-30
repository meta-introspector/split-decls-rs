// Generated macro for macro_1077 (macro)
macro_rules! Depcrate_castsmacro_1077 {
() => {
// Module: crate::casts
// Provides: {"macro_1077"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for casts from any numeric type to a float type where"] # [doc = " the receiving type cannot store all values from the original type without"] # [doc = " rounding errors. This possible rounding is to be expected, so this lint is"] # [doc = " `Allow` by default."] # [doc = ""] # [doc = " Basically, this warns on casting any integer with 32 or more bits to `f32`"] # [doc = " or any 64-bit integer to `f64`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's not bad at all. But in some applications it can be"] # [doc = " helpful to know where precision loss can take place. This lint can help find"] # [doc = " those places in the code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = u64::MAX;"] # [doc = " x as f64;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub CAST_PRECISION_LOSS , pedantic , "casts that cause loss of precision, e.g., `x as f32` where `x: u64`" }
};
}

// Generated macro for macro_1081 (macro)
macro_rules! Depcrate_castsmacro_1081 {
() => {
// Module: crate::casts
// Provides: {"macro_1081"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for casts between numeric types that can be replaced by safe"] # [doc = " conversion functions."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Rust's `as` keyword will perform many kinds of conversions, including"] # [doc = " silently lossy conversions. Conversion functions such as `i32::from`"] # [doc = " will only perform lossless conversions. Using the conversion functions"] # [doc = " prevents conversions from becoming silently lossy if the input types"] # [doc = " ever change, and makes it clear for people reading the code that the"] # [doc = " conversion is lossless."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn as_u64(x: u8) -> u64 {"] # [doc = "     x as u64"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Using `::from` would look like this:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " fn as_u64(x: u8) -> u64 {"] # [doc = "     u64::from(x)"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub CAST_LOSSLESS , pedantic , "casts using `as` that are known to be lossless, e.g., `x as u64` where `x: u8`" }
};
}

// Generated macro for macro_1066 (macro)
macro_rules! Depcrate_castsmacro_1066 {
() => {
// Module: crate::casts
// Provides: {"macro_1066"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for casts between numeric types that may"] # [doc = " truncate large values. This is expected behavior, so the cast is `Allow` by"] # [doc = " default. It suggests user either explicitly ignore the lint,"] # [doc = " or use `try_from()` and handle the truncation, default, or panic explicitly."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " In some problem domains, it is good practice to avoid"] # [doc = " truncation. This lint can be activated to help assess where additional"] # [doc = " checks could be beneficial."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn as_u8(x: u64) -> u8 {"] # [doc = "     x as u8"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn as_u8(x: u64) -> u8 {"] # [doc = "     if let Ok(x) = u8::try_from(x) {"] # [doc = "         x"] # [doc = "     } else {"] # [doc = "         todo!();"] # [doc = "     }"] # [doc = " }"] # [doc = " // Or"] # [doc = " #[allow(clippy::cast_possible_truncation)]"] # [doc = " fn as_u16(x: u64) -> u16 {"] # [doc = "     x as u16"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub CAST_POSSIBLE_TRUNCATION , pedantic , "casts that may cause truncation of the value, e.g., `x as u8` where `x: u32`, or `x as i32` where `x: f32`" }
};
}

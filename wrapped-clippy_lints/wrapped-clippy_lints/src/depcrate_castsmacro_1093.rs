// Generated macro for macro_1093 (macro)
macro_rules! Depcrate_castsmacro_1093 {
() => {
// Module: crate::casts
// Provides: {"macro_1093"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of the `abs()` method that cast the result to unsigned."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The `unsigned_abs()` method avoids panic when called on the MIN value."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x: i32 = -42;"] # [doc = " let y: u32 = x.abs() as u32;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x: i32 = -42;"] # [doc = " let y: u32 = x.unsigned_abs();"] # [doc = " ```"] # [clippy :: version = "1.62.0"] pub CAST_ABS_TO_UNSIGNED , suspicious , "casting the result of `abs()` to an unsigned integer can panic" }
};
}

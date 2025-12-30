// Generated macro for macro_8758 (macro)
macro_rules! Depcrate_operatorsmacro_8758 {
() => {
// Module: crate::operators
// Provides: {"macro_8758"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for an expression like `(x + (y - 1)) / y` which is a common manual reimplementation"] # [doc = " of `x.div_ceil(y)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's simpler, clearer and more readable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x: i32 = 7;"] # [doc = " let y: i32 = 4;"] # [doc = " let div = (x + (y - 1)) / y;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #![feature(int_roundings)]"] # [doc = " let x: i32 = 7;"] # [doc = " let y: i32 = 4;"] # [doc = " let div = x.div_ceil(y);"] # [doc = " ```"] # [clippy :: version = "1.83.0"] pub MANUAL_DIV_CEIL , complexity , "manually reimplementing `div_ceil`" }
};
}

// Generated macro for macro_8748 (macro)
macro_rules! Depcrate_operatorsmacro_8748 {
() => {
// Module: crate::operators
// Provides: {"macro_8748"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for division of integers"] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " When outside of some very specific algorithms,"] # [doc = " integer division is very often a mistake because it discards the"] # [doc = " remainder."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = 3 / 2;"] # [doc = " println!(\"{}\", x);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = 3f32 / 2f32;"] # [doc = " println!(\"{}\", x);"] # [doc = " ```"] # [clippy :: version = "1.37.0"] pub INTEGER_DIVISION , restriction , "integer division may cause loss of precision" }
};
}

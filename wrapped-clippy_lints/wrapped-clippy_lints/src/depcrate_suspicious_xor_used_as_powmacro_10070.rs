// Generated macro for macro_10070 (macro)
macro_rules! Depcrate_suspicious_xor_used_as_powmacro_10070 {
() => {
// Module: crate::suspicious_xor_used_as_pow
// Provides: {"macro_10070"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns for a Bitwise XOR (`^`) operator being probably confused as a powering. It will not trigger if any of the numbers are not in decimal."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " It's most probably a typo and may lead to unexpected behaviours."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = 3_i32 ^ 4_i32;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = 3_i32.pow(4);"] # [doc = " ```"] # [clippy :: version = "1.67.0"] pub SUSPICIOUS_XOR_USED_AS_POW , restriction , "XOR (`^`) operator possibly used as exponentiation operator" }
};
}

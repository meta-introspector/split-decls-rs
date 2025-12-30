// Generated macro for macro_3023 (macro)
macro_rules! Depcrate_integer_division_remainder_usedmacro_3023 {
() => {
// Module: crate::integer_division_remainder_used
// Provides: {"macro_3023"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the usage of division (`/`) and remainder (`%`) operations"] # [doc = " when performed on any integer types using the default `Div` and `Rem` trait implementations."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " In cryptographic contexts, division can result in timing sidechannel vulnerabilities,"] # [doc = " and needs to be replaced with constant-time code instead (e.g. Barrett reduction)."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let my_div = 10 / 2;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let my_div = 10 >> 1;"] # [doc = " ```"] # [clippy :: version = "1.79.0"] pub INTEGER_DIVISION_REMAINDER_USED , restriction , "use of disallowed default division and remainder operations" }
};
}

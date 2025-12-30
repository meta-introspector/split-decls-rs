// Generated macro for macro_3422 (macro)
macro_rules! Depcrate_literal_representationmacro_3422 {
() => {
// Module: crate::literal_representation
// Provides: {"macro_3422"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns if there is a better representation for a numeric literal."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Especially for big powers of 2, a hexadecimal representation is usually more"] # [doc = " readable than a decimal representation."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```text"] # [doc = " `255` => `0xFF`"] # [doc = " `65_535` => `0xFFFF`"] # [doc = " `4_042_322_160` => `0xF0F0_F0F0`"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub DECIMAL_LITERAL_REPRESENTATION , restriction , "using decimal representation when hexadecimal would be better" }
};
}

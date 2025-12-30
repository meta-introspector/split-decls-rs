// Generated macro for macro_3418 (macro)
macro_rules! Depcrate_literal_representationmacro_3418 {
() => {
// Module: crate::literal_representation
// Provides: {"macro_3418"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns for mistyped suffix in literals"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is most probably a typo"] # [doc = ""] # [doc = " ### Known problems"] # [doc = " - Does not match on integers too large to fit in the corresponding unsigned type"] # [doc = " - Does not match on `_127` since that is a valid grouping for decimal and octal numbers"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " `2_32` => `2_i32`"] # [doc = " `250_8 => `250_u8`"] # [doc = " ```"] # [clippy :: version = "1.30.0"] pub MISTYPED_LITERAL_SUFFIXES , correctness , "mistyped literal suffix" }
};
}

// Generated macro for macro_3467 (macro)
macro_rules! Depcrate_literal_representationmacro_3467 {
() => {
// Module: crate::literal_representation
// Provides: {"macro_3467"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns if hexadecimal or binary literals are not grouped"] # [doc = " by nibble or byte."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Negatively impacts readability."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x: u32 = 0xFFF_FFF;"] # [doc = " let y: u8 = 0b01_011_101;"] # [doc = " ```"] # [clippy :: version = "1.49.0"] pub UNUSUAL_BYTE_GROUPINGS , style , "binary or hex literals that aren't grouped by four" }
};
}

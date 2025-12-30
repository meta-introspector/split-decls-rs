// Generated macro for macro_2201 (macro)
macro_rules! Depcrate_float_literalmacro_2201 {
() => {
// Module: crate::float_literal
// Provides: {"macro_2201"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for whole number float literals that"] # [doc = " cannot be represented as the underlying type without loss."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " If the value was intended to be exact, it will not be."] # [doc = " This may be especially surprising when the lost precision is to the left of the decimal point."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _: f32 = 16_777_217.0; // 16_777_216.0"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let _: f32 = 16_777_216.0;"] # [doc = " let _: f64 = 16_777_217.0;"] # [doc = " ```"] # [clippy :: version = "1.43.0"] pub LOSSY_FLOAT_LITERAL , restriction , "lossy whole number float literals" }
};
}

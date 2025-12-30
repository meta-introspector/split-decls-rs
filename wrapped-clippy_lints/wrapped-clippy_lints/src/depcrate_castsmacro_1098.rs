// Generated macro for macro_1098 (macro)
macro_rules! Depcrate_castsmacro_1098 {
() => {
// Module: crate::casts
// Provides: {"macro_1098"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for a known NaN float being cast to an integer"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " NaNs are cast into zero, so one could simply use this and make the"] # [doc = " code more readable. The lint could also hint at a programmer error."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let _ = (0.0_f32 / 0.0) as u64;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " let _ = 0_u64;"] # [doc = " ```"] # [clippy :: version = "1.66.0"] pub CAST_NAN_TO_INT , suspicious , "casting a known floating-point NaN into an integer" }
};
}

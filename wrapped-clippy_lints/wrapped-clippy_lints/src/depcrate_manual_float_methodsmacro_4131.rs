// Generated macro for macro_4131 (macro)
macro_rules! Depcrate_manual_float_methodsmacro_4131 {
() => {
// Module: crate::manual_float_methods
// Provides: {"macro_4131"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for manual `is_finite` reimplementations"] # [doc = " (i.e., `x != <float>::INFINITY && x != <float>::NEG_INFINITY`)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The method `is_finite` is shorter and more readable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let x = 1.0f32;"] # [doc = " if x != f32::INFINITY && x != f32::NEG_INFINITY {}"] # [doc = " if x.abs() < f32::INFINITY {}"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let x = 1.0f32;"] # [doc = " if x.is_finite() {}"] # [doc = " if x.is_finite() {}"] # [doc = " ```"] # [clippy :: version = "1.73.0"] pub MANUAL_IS_FINITE , style , "use dedicated method to check if a float is finite" }
};
}

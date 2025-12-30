// Generated macro for macro_2197 (macro)
macro_rules! Depcrate_floating_point_arithmeticmacro_2197 {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"macro_2197"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Looks for floating-point expressions that"] # [doc = " can be expressed using built-in methods to improve accuracy"] # [doc = " at the cost of performance."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Negatively impacts accuracy."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let a = 3f32;"] # [doc = " let _ = a.powf(1.0 / 3.0);"] # [doc = " let _ = (1.0 + a).ln();"] # [doc = " let _ = a.exp() - 1.0;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let a = 3f32;"] # [doc = " let _ = a.cbrt();"] # [doc = " let _ = a.ln_1p();"] # [doc = " let _ = a.exp_m1();"] # [doc = " ```"] # [clippy :: version = "1.43.0"] pub IMPRECISE_FLOPS , nursery , "usage of imprecise floating point operations" }
};
}

// Generated macro for macro_2198 (macro)
macro_rules! Depcrate_floating_point_arithmeticmacro_2198 {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"macro_2198"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Looks for floating-point expressions that"] # [doc = " can be expressed using built-in methods to improve both"] # [doc = " accuracy and performance."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Negatively impacts accuracy and performance."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::f32::consts::E;"] # [doc = ""] # [doc = " let a = 3f32;"] # [doc = " let _ = (2f32).powf(a);"] # [doc = " let _ = E.powf(a);"] # [doc = " let _ = a.powf(1.0 / 2.0);"] # [doc = " let _ = a.log(2.0);"] # [doc = " let _ = a.log(10.0);"] # [doc = " let _ = a.log(E);"] # [doc = " let _ = a.powf(2.0);"] # [doc = " let _ = a * 2.0 + 4.0;"] # [doc = " let _ = if a < 0.0 {"] # [doc = "     -a"] # [doc = " } else {"] # [doc = "     a"] # [doc = " };"] # [doc = " let _ = if a < 0.0 {"] # [doc = "     a"] # [doc = " } else {"] # [doc = "     -a"] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " is better expressed as"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::f32::consts::E;"] # [doc = ""] # [doc = " let a = 3f32;"] # [doc = " let _ = a.exp2();"] # [doc = " let _ = a.exp();"] # [doc = " let _ = a.sqrt();"] # [doc = " let _ = a.log2();"] # [doc = " let _ = a.log10();"] # [doc = " let _ = a.ln();"] # [doc = " let _ = a.powi(2);"] # [doc = " let _ = a.mul_add(2.0, 4.0);"] # [doc = " let _ = a.abs();"] # [doc = " let _ = -a.abs();"] # [doc = " ```"] # [clippy :: version = "1.43.0"] pub SUBOPTIMAL_FLOPS , nursery , "usage of sub-optimal floating point operations" }
};
}

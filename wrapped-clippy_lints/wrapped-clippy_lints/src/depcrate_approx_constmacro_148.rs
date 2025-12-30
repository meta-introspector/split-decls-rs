// Generated macro for macro_148 (macro)
macro_rules! Depcrate_approx_constmacro_148 {
() => {
// Module: crate::approx_const
// Provides: {"macro_148"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for floating point literals that approximate"] # [doc = " constants which are defined in"] # [doc = " [`std::f32::consts`](https://doc.rust-lang.org/stable/std/f32/consts/#constants)"] # [doc = " or"] # [doc = " [`std::f64::consts`](https://doc.rust-lang.org/stable/std/f64/consts/#constants),"] # [doc = " respectively, suggesting to use the predefined constant."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Usually, the definition in the standard library is more"] # [doc = " precise than what people come up with. If you find that your definition is"] # [doc = " actually more precise, please [file a Rust"] # [doc = " issue](https://github.com/rust-lang/rust/issues)."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = 3.14;"] # [doc = " let y = 1_f64 / x;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = std::f32::consts::PI;"] # [doc = " let y = std::f64::consts::FRAC_1_PI;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub APPROX_CONSTANT , correctness , "the approximate of a known float constant (in `std::fXX::consts`)" }
};
}

// Generated macro for macro_11522 (macro)
macro_rules! Depcrate_zero_div_zeromacro_11522 {
() => {
// Module: crate::zero_div_zero
// Provides: {"macro_11522"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `0.0 / 0.0`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's less readable than `f32::NAN` or `f64::NAN`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let nan = 0.0f32 / 0.0;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let nan = f32::NAN;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub ZERO_DIVIDED_BY_ZERO , complexity , "usage of `0.0 / 0.0` to obtain NaN instead of `f32::NAN` or `f64::NAN`" }
};
}

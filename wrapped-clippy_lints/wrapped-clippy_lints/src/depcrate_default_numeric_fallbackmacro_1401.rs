// Generated macro for macro_1401 (macro)
macro_rules! Depcrate_default_numeric_fallbackmacro_1401 {
() => {
// Module: crate::default_numeric_fallback
// Provides: {"macro_1401"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of unconstrained numeric literals which may cause default numeric fallback in type"] # [doc = " inference."] # [doc = ""] # [doc = " Default numeric fallback means that if numeric types have not yet been bound to concrete"] # [doc = " types at the end of type inference, then integer type is bound to `i32`, and similarly"] # [doc = " floating type is bound to `f64`."] # [doc = ""] # [doc = " See [RFC0212](https://github.com/rust-lang/rfcs/blob/master/text/0212-restore-int-fallback.md) for more information about the fallback."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " To ensure that every numeric type is chosen explicitly rather than implicitly."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " This lint is implemented using a custom algorithm independent of rustc's inference,"] # [doc = " which results in many false positives and false negatives."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let i = 10;"] # [doc = " let f = 1.23;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let i = 10_i32;"] # [doc = " let f = 1.23_f64;"] # [doc = " ```"] # [clippy :: version = "1.52.0"] pub DEFAULT_NUMERIC_FALLBACK , restriction , "usage of unconstrained numeric literals which may cause default numeric fallback." }
};
}

// Generated macro for macro_10761 (macro)
macro_rules! Depcrate_unused_roundingmacro_10761 {
() => {
// Module: crate::unused_rounding
// Provides: {"macro_10761"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Detects cases where a whole-number literal float is being rounded, using"] # [doc = " the `floor`, `ceil`, or `round` methods."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " This is unnecessary and confusing to the reader. Doing this is probably a mistake."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = 1f32.ceil();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = 1f32;"] # [doc = " ```"] # [clippy :: version = "1.63.0"] pub UNUSED_ROUNDING , nursery , "Uselessly rounding a whole number floating-point literal" }
};
}

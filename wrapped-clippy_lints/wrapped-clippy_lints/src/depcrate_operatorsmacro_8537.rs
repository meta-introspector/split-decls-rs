// Generated macro for macro_8537 (macro)
macro_rules! Depcrate_operatorsmacro_8537 {
() => {
// Module: crate::operators
// Provides: {"macro_8537"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for statements of the form `(a - b) < f32::EPSILON` or"] # [doc = " `(a - b) < f64::EPSILON`. Notes the missing `.abs()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The code without `.abs()` is more likely to have a bug."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " If the user can ensure that b is larger than a, the `.abs()` is"] # [doc = " technically unnecessary. However, it will make the code more robust and doesn't have any"] # [doc = " large performance implications. If the abs call was deliberately left out for performance"] # [doc = " reasons, it is probably better to state this explicitly in the code, which then can be done"] # [doc = " with an allow."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " pub fn is_roughly_equal(a: f32, b: f32) -> bool {"] # [doc = "     (a - b) < f32::EPSILON"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " pub fn is_roughly_equal(a: f32, b: f32) -> bool {"] # [doc = "     (a - b).abs() < f32::EPSILON"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.48.0"] pub FLOAT_EQUALITY_WITHOUT_ABS , suspicious , "float equality check without `.abs()`" }
};
}

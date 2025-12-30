// Generated macro for macro_458 (macro)
macro_rules! Depcrate_attrsmacro_458 {
() => {
// Module: crate::attrs
// Provides: {"macro_458"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `any` and `all` combinators in `cfg` with only one condition."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " If there is only one condition, no need to wrap it into `any` or `all` combinators."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #[cfg(any(unix))]"] # [doc = " pub struct Bar;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[cfg(unix)]"] # [doc = " pub struct Bar;"] # [doc = " ```"] # [clippy :: version = "1.71.0"] pub NON_MINIMAL_CFG , style , "ensure that all `cfg(any())` and `cfg(all())` have more than one condition" }
};
}

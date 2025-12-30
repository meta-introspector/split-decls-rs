// Generated macro for macro_99 (macro)
macro_rules! Depcrate_lint_without_lint_passmacro_99 {
() => {
// Module: crate::lint_without_lint_pass
// Provides: {"macro_99"}
// Dependencies: {}
declare_tool_lint ! { # [doc = " ### What it does"] # [doc = " Checks for declared clippy lints without the `clippy::version` attribute."] pub clippy :: MISSING_CLIPPY_VERSION_ATTRIBUTE , Warn , "found clippy lint without `clippy::version` attribute" , report_in_external_macro : true }
};
}

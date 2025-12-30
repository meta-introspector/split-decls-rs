// Generated macro for macro_98 (macro)
macro_rules! Depcrate_lint_without_lint_passmacro_98 {
() => {
// Module: crate::lint_without_lint_pass
// Provides: {"macro_98"}
// Dependencies: {}
declare_tool_lint ! { # [doc = " ### What it does"] # [doc = " Checks for invalid `clippy::version` attributes."] # [doc = ""] # [doc = " Valid values are:"] # [doc = " * \"pre 1.29.0\""] # [doc = " * any valid semantic version"] pub clippy :: INVALID_CLIPPY_VERSION_ATTRIBUTE , Warn , "found an invalid `clippy::version` attribute" , report_in_external_macro : true }
};
}

// Generated macro for macro_207 (macro)
macro_rules! Depcrate_unusual_namesmacro_207 {
() => {
// Module: crate::unusual_names
// Provides: {"macro_207"}
// Dependencies: {}
declare_tool_lint ! { # [doc = " ### What it does"] # [doc = " Checks if variables of some types use the usual name."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Restricting the identifiers used for common things in"] # [doc = " Clippy sources increases consistency."] # [doc = ""] # [doc = " ### Example"] # [doc = " Check that an `rustc_errors::Applicability` variable is"] # [doc = " named either `app` or `applicability`, and not"] # [doc = " `a` or `appl`."] pub clippy :: UNUSUAL_NAMES , Warn , "commonly used concepts should use usual same variable name." , report_in_external_macro : true }
};
}

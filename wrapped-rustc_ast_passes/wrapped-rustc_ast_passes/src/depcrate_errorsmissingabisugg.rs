// Generated macro for MissingAbiSugg (struct)
macro_rules! Depcrate_errorsMissingAbiSugg {
() => {
// Module: crate::errors
// Provides: {"MissingAbiSugg"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (ast_passes_extern_without_abi_sugg)] pub (crate) struct MissingAbiSugg { # [suggestion (code = "extern {default_abi}" , applicability = "machine-applicable")] pub span : Span , pub default_abi : ExternAbi , }
};
}

// Generated macro for InvalidTargetLint (struct)
macro_rules! Depcrate_session_diagnosticsInvalidTargetLint {
() => {
// Module: crate::session_diagnostics
// Provides: {"InvalidTargetLint"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (attr_parsing_invalid_target_lint)] # [warning] # [help] pub (crate) struct InvalidTargetLint { pub name : AttrPath , pub target : & 'static str , pub applied : DiagArgValue , pub only : & 'static str , # [suggestion (code = "" , applicability = "machine-applicable" , style = "tool-only")] pub attr_span : Span , }
};
}

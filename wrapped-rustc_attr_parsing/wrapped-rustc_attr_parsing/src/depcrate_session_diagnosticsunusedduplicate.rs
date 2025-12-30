// Generated macro for UnusedDuplicate (struct)
macro_rules! Depcrate_session_diagnosticsUnusedDuplicate {
() => {
// Module: crate::session_diagnostics
// Provides: {"UnusedDuplicate"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (attr_parsing_unused_duplicate)] pub (crate) struct UnusedDuplicate { # [suggestion (code = "" , applicability = "machine-applicable")] pub this : Span , # [note] pub other : Span , # [warning] pub warning : bool , }
};
}

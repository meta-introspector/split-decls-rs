// Generated macro for TildeConstDisallowed (struct)
macro_rules! Depcrate_errorsTildeConstDisallowed {
() => {
// Module: crate::errors
// Provides: {"TildeConstDisallowed"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_tilde_const_disallowed)] pub (crate) struct TildeConstDisallowed { # [primary_span] pub span : Span , # [subdiagnostic] pub reason : TildeConstReason , }
};
}

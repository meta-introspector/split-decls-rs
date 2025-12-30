// Generated macro for UnconstrainedOpaqueType (struct)
macro_rules! Depcrate_errorsUnconstrainedOpaqueType {
() => {
// Module: crate::errors
// Provides: {"UnconstrainedOpaqueType"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_unconstrained_opaque_type)] # [note] pub (crate) struct UnconstrainedOpaqueType { # [primary_span] pub span : Span , pub name : Ident , pub what : & 'static str , }
};
}

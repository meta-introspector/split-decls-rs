// Generated macro for LifetimeMismatchOpaqueParam (struct)
macro_rules! Depcrate_session_diagnosticsLifetimeMismatchOpaqueParam {
() => {
// Module: crate::session_diagnostics
// Provides: {"LifetimeMismatchOpaqueParam"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (borrowck_opaque_type_lifetime_mismatch)] pub (crate) struct LifetimeMismatchOpaqueParam < 'tcx > { pub arg : GenericArg < 'tcx > , pub prev : GenericArg < 'tcx > , # [primary_span] # [label] # [note] pub span : Span , # [label (borrowck_prev_lifetime_label)] pub prev_span : Span , }
};
}

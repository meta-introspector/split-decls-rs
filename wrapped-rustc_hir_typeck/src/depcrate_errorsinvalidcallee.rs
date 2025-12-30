// Generated macro for InvalidCallee (struct)
macro_rules! Depcrate_errorsInvalidCallee {
() => {
// Module: crate::errors
// Provides: {"InvalidCallee"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_invalid_callee , code = E0618)] pub (crate) struct InvalidCallee < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub found : String , }
};
}

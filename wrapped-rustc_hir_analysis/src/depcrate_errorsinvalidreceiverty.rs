// Generated macro for InvalidReceiverTy (struct)
macro_rules! Depcrate_errorsInvalidReceiverTy {
() => {
// Module: crate::errors
// Provides: {"InvalidReceiverTy"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_invalid_receiver_ty , code = E0307)] # [note] # [help (hir_analysis_invalid_receiver_ty_help)] pub (crate) struct InvalidReceiverTy < 'tcx > { # [primary_span] pub span : Span , pub receiver_ty : Ty < 'tcx > , # [subdiagnostic] pub hint : Option < InvalidReceiverTyHint > , }
};
}

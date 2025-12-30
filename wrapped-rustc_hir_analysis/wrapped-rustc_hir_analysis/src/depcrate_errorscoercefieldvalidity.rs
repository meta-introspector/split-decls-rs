// Generated macro for CoerceFieldValidity (struct)
macro_rules! Depcrate_errorsCoerceFieldValidity {
() => {
// Module: crate::errors
// Provides: {"CoerceFieldValidity"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_coerce_unsized_field_validity)] pub (crate) struct CoerceFieldValidity < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub trait_name : & 'static str , # [label] pub field_span : Span , pub field_ty : Ty < 'tcx > , }
};
}

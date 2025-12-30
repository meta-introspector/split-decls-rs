// Generated macro for CoerceNoField (struct)
macro_rules! Depcrate_errorsCoerceNoField {
() => {
// Module: crate::errors
// Provides: {"CoerceNoField"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_coerce_zero , code = E0374)] pub (crate) struct CoerceNoField { # [primary_span] pub span : Span , pub trait_name : & 'static str , # [note (hir_analysis_coercion_between_struct_single_note)] pub note : bool , }
};
}

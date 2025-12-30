// Generated macro for InvalidUnionField (struct)
macro_rules! Depcrate_errorsInvalidUnionField {
() => {
// Module: crate::errors
// Provides: {"InvalidUnionField"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_invalid_union_field , code = E0740)] pub (crate) struct InvalidUnionField { # [primary_span] pub field_span : Span , # [subdiagnostic] pub sugg : InvalidUnionFieldSuggestion , # [note] pub note : () , }
};
}

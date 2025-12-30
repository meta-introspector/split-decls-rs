// Generated macro for ReturnTypeNotationOnNonRpitit (struct)
macro_rules! Depcrate_errorsReturnTypeNotationOnNonRpitit {
() => {
// Module: crate::errors
// Provides: {"ReturnTypeNotationOnNonRpitit"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_return_type_notation_on_non_rpitit)] pub (crate) struct ReturnTypeNotationOnNonRpitit < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , # [label] pub fn_span : Option < Span > , # [note] pub note : () , }
};
}

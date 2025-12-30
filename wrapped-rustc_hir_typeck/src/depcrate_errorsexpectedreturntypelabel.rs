// Generated macro for ExpectedReturnTypeLabel (enum)
macro_rules! Depcrate_errorsExpectedReturnTypeLabel {
() => {
// Module: crate::errors
// Provides: {"ExpectedReturnTypeLabel"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum ExpectedReturnTypeLabel < 'tcx > { # [label (hir_typeck_expected_default_return_type)] Unit { # [primary_span] span : Span , } , # [label (hir_typeck_expected_return_type)] Other { # [primary_span] span : Span , expected : Ty < 'tcx > , } , }
};
}

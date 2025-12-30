// Generated macro for DropImplPolarity (enum)
macro_rules! Depcrate_errorsDropImplPolarity {
() => {
// Module: crate::errors
// Provides: {"DropImplPolarity"}
// Dependencies: {}
# [derive (Diagnostic)] pub (crate) enum DropImplPolarity { # [diag (hir_analysis_drop_impl_negative)] Negative { # [primary_span] span : Span , } , # [diag (hir_analysis_drop_impl_reservation)] Reservation { # [primary_span] span : Span , } , }
};
}

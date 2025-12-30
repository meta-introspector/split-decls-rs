// Generated macro for impl_251 (impl)
macro_rules! Depcrate_errorsimpl_251 {
() => {
// Module: crate::errors
// Provides: {"impl_251"}
// Dependencies: {}
impl Subdiagnostic for RemoveSemiForCoerce { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { let mut multispan : MultiSpan = self . semi . into () ; multispan . push_span_label (self . expr , fluent :: hir_typeck_remove_semi_for_coerce_expr) ; multispan . push_span_label (self . ret , fluent :: hir_typeck_remove_semi_for_coerce_ret) ; multispan . push_span_label (self . semi , fluent :: hir_typeck_remove_semi_for_coerce_semi) ; diag . span_note (multispan , fluent :: hir_typeck_remove_semi_for_coerce) ; diag . tool_only_span_suggestion (self . semi , fluent :: hir_typeck_remove_semi_for_coerce_suggestion , "" , Applicability :: MaybeIncorrect ,) ; } }
};
}

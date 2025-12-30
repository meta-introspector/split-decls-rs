// Generated macro for impl_233 (impl)
macro_rules! Depcrate_errorsimpl_233 {
() => {
// Module: crate::errors
// Provides: {"impl_233"}
// Dependencies: {}
impl Subdiagnostic for TypeMismatchFruTypo { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . arg ("expr" , self . expr . as_deref () . unwrap_or ("NONE")) ; if self . expr_span . between (self . fru_span) . is_empty () { diag . span_note (self . expr_span . to (self . fru_span) , fluent :: hir_typeck_fru_note) ; } else { let mut multispan : MultiSpan = vec ! [self . expr_span , self . fru_span] . into () ; multispan . push_span_label (self . expr_span , fluent :: hir_typeck_fru_expr) ; multispan . push_span_label (self . fru_span , fluent :: hir_typeck_fru_expr2) ; diag . span_note (multispan , fluent :: hir_typeck_fru_note) ; } diag . span_suggestion (self . expr_span . shrink_to_hi () , fluent :: hir_typeck_fru_suggestion , ", " , Applicability :: MaybeIncorrect ,) ; } }
};
}

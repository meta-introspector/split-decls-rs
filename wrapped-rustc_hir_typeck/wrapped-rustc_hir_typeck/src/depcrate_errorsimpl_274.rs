// Generated macro for impl_274 (impl)
macro_rules! Depcrate_errorsimpl_274 {
() => {
// Module: crate::errors
// Provides: {"impl_274"}
// Dependencies: {}
impl rustc_errors :: Subdiagnostic for CastUnknownPointerSub { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { match self { CastUnknownPointerSub :: To (span) => { let msg = diag . eagerly_translate (fluent :: hir_typeck_label_to) ; diag . span_label (span , msg) ; let msg = diag . eagerly_translate (fluent :: hir_typeck_note) ; diag . note (msg) ; } CastUnknownPointerSub :: From (span) => { let msg = diag . eagerly_translate (fluent :: hir_typeck_label_from) ; diag . span_label (span , msg) ; } } } }
};
}

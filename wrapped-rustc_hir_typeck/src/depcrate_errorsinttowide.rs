// Generated macro for IntToWide (struct)
macro_rules! Depcrate_errorsIntToWide {
() => {
// Module: crate::errors
// Provides: {"IntToWide"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_int_to_fat , code = E0606)] pub (crate) struct IntToWide < 'tcx > { # [primary_span] # [label (hir_typeck_int_to_fat_label)] pub span : Span , pub metadata : & 'tcx str , pub expr_ty : Ty < 'tcx > , pub cast_ty : Ty < 'tcx > , # [label (hir_typeck_int_to_fat_label_nightly)] pub expr_if_nightly : Option < Span > , pub known_wide : bool , }
};
}

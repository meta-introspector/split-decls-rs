// Generated macro for CastThinPointerToWidePointer (struct)
macro_rules! Depcrate_errorsCastThinPointerToWidePointer {
() => {
// Module: crate::errors
// Provides: {"CastThinPointerToWidePointer"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_cast_thin_pointer_to_wide_pointer , code = E0607)] pub (crate) struct CastThinPointerToWidePointer < 'tcx > { # [primary_span] pub span : Span , pub expr_ty : Ty < 'tcx > , pub cast_ty : Ty < 'tcx > , # [note (hir_typeck_teach_help)] pub (crate) teach : bool , }
};
}

// Generated macro for CastEnumDrop (struct)
macro_rules! Depcrate_errorsCastEnumDrop {
() => {
// Module: crate::errors
// Provides: {"CastEnumDrop"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_cast_enum_drop)] pub (crate) struct CastEnumDrop < 'tcx > { # [primary_span] pub span : Span , pub expr_ty : Ty < 'tcx > , pub cast_ty : Ty < 'tcx > , }
};
}

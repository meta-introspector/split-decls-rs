macro_rules! CastEnumDrop {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_cast_enum_drop)] pub (crate) struct CastEnumDrop < 'tcx > { # [primary_span] pub span : Span , pub expr_ty : Ty < 'tcx > , pub cast_ty : Ty < 'tcx > , }
    };
}

CastEnumDrop!();
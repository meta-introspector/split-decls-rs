macro_rules! NoVariantNamed {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_no_variant_named , code = E0599)] pub struct NoVariantNamed < 'tcx > { # [primary_span] pub span : Span , pub ident : Ident , pub ty : Ty < 'tcx > , }
    };
}

NoVariantNamed!();
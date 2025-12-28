macro_rules! TypeOf {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_type_of)] pub (crate) struct TypeOf < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , }
    };
}

TypeOf!();
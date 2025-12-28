macro_rules! VariancesOf {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_variances_of)] pub (crate) struct VariancesOf { # [primary_span] pub span : Span , pub variances : String , }
    };
}

VariancesOf!();
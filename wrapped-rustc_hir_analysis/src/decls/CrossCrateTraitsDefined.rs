macro_rules! CrossCrateTraitsDefined {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_cross_crate_traits_defined , code = E0321)] pub (crate) struct CrossCrateTraitsDefined { # [primary_span] # [label] pub span : Span , pub traits : String , }
    };
}

CrossCrateTraitsDefined!();
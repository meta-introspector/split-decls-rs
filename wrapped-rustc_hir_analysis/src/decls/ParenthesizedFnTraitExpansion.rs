macro_rules! ParenthesizedFnTraitExpansion {
    () => {
        # [derive (Subdiagnostic)] # [help (hir_analysis_parenthesized_fn_trait_expansion)] pub (crate) struct ParenthesizedFnTraitExpansion { # [primary_span] pub span : Span , pub expanded_type : String , }
    };
}

ParenthesizedFnTraitExpansion!()
macro_rules! TraitObjectBound {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_trait_object_single_bound , code = E0226)] pub (crate) struct TraitObjectBound { # [primary_span] pub span : Span , }
    };
}

TraitObjectBound!();
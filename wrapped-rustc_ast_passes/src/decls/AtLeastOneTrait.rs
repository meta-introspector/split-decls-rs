macro_rules! AtLeastOneTrait {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_at_least_one_trait)] pub (crate) struct AtLeastOneTrait { # [primary_span] pub span : Span , }
    };
}

AtLeastOneTrait!()
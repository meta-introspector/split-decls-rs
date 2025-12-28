macro_rules! NonIdentFeature {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_non_ident_feature , code = E0546)] pub (crate) struct NonIdentFeature { # [primary_span] pub span : Span , }
    };
}

NonIdentFeature!()
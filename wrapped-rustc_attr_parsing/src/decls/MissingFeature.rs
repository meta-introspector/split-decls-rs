macro_rules! MissingFeature {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_missing_feature , code = E0546)] pub (crate) struct MissingFeature { # [primary_span] pub span : Span , }
    };
}

MissingFeature!();
macro_rules! InvalidSince {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_invalid_since)] pub (crate) struct InvalidSince { # [primary_span] pub span : Span , }
    };
}

InvalidSince!();
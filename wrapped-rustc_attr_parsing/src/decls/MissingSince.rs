macro_rules! MissingSince {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_missing_since , code = E0542)] pub (crate) struct MissingSince { # [primary_span] pub span : Span , }
    };
}

MissingSince!();
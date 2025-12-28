macro_rules! MissingNote {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_missing_note , code = E0543)] pub (crate) struct MissingNote { # [primary_span] pub span : Span , }
    };
}

MissingNote!()
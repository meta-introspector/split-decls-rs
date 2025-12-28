macro_rules! MissingIssue {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_missing_issue , code = E0547)] pub (crate) struct MissingIssue { # [primary_span] pub span : Span , }
    };
}

MissingIssue!()
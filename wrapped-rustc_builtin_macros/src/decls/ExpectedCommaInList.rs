macro_rules! ExpectedCommaInList {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_expected_comma_in_list)] pub (crate) struct ExpectedCommaInList { # [primary_span] pub span : Span , }
    };
}

ExpectedCommaInList!()
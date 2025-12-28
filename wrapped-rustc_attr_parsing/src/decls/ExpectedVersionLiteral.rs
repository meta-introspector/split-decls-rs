macro_rules! ExpectedVersionLiteral {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_expected_version_literal)] pub (crate) struct ExpectedVersionLiteral { # [primary_span] pub span : Span , }
    };
}

ExpectedVersionLiteral!()
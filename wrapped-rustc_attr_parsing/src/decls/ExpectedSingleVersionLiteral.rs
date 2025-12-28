macro_rules! ExpectedSingleVersionLiteral {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_expected_single_version_literal)] pub (crate) struct ExpectedSingleVersionLiteral { # [primary_span] pub span : Span , }
    };
}

ExpectedSingleVersionLiteral!()
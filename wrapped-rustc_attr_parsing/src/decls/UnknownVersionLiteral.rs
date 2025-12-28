macro_rules! UnknownVersionLiteral {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_unknown_version_literal)] pub (crate) struct UnknownVersionLiteral { # [primary_span] pub span : Span , }
    };
}

UnknownVersionLiteral!()
macro_rules! SuffixedLiteralInAttribute {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_suffixed_literal_in_attribute)] # [help] pub (crate) struct SuffixedLiteralInAttribute { # [primary_span] pub span : Span , }
    };
}

SuffixedLiteralInAttribute!()
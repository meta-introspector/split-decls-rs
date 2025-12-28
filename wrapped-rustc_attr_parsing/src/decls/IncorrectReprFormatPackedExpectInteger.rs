macro_rules! IncorrectReprFormatPackedExpectInteger {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_incorrect_repr_format_packed_expect_integer , code = E0552)] pub (crate) struct IncorrectReprFormatPackedExpectInteger { # [primary_span] pub span : Span , }
    };
}

IncorrectReprFormatPackedExpectInteger!()
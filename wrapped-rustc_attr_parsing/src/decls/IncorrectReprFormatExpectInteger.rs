macro_rules! IncorrectReprFormatExpectInteger {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_incorrect_repr_format_expect_literal_integer , code = E0693)] pub (crate) struct IncorrectReprFormatExpectInteger { # [primary_span] pub span : Span , }
    };
}

IncorrectReprFormatExpectInteger!();
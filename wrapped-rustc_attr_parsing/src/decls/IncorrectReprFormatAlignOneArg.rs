macro_rules! IncorrectReprFormatAlignOneArg {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_incorrect_repr_format_align_one_arg , code = E0693)] pub (crate) struct IncorrectReprFormatAlignOneArg { # [primary_span] pub span : Span , }
    };
}

IncorrectReprFormatAlignOneArg!();
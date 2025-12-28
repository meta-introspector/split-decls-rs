macro_rules! deps {
    () => {
        IncorrectReprFormatGenericCause!();
    };
}

macro_rules! IncorrectReprFormatGeneric {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (attr_parsing_incorrect_repr_format_generic , code = E0693)] pub (crate) struct IncorrectReprFormatGeneric { # [primary_span] pub span : Span , pub repr_arg : Symbol , # [subdiagnostic] pub cause : Option < IncorrectReprFormatGenericCause > , }
    };
}

IncorrectReprFormatGeneric!()
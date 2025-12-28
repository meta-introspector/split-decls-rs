macro_rules! InvalidReprGeneric {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_invalid_repr_generic , code = E0589)] pub (crate) struct InvalidReprGeneric < 'a > { # [primary_span] pub span : Span , pub repr_arg : String , pub error_part : & 'a str , }
    };
}

InvalidReprGeneric!()
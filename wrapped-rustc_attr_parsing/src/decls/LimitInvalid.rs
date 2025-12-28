macro_rules! LimitInvalid {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_limit_invalid)] pub (crate) struct LimitInvalid < 'a > { # [primary_span] pub span : Span , # [label] pub value_span : Span , pub error_str : & 'a str , }
    };
}

LimitInvalid!()
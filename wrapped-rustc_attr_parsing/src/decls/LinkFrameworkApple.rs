macro_rules! LinkFrameworkApple {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_link_framework_apple , code = E0455)] pub (crate) struct LinkFrameworkApple { # [primary_span] pub span : Span , }
    };
}

LinkFrameworkApple!()
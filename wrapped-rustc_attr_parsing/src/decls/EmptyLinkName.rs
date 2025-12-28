macro_rules! EmptyLinkName {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_empty_link_name , code = E0454)] pub (crate) struct EmptyLinkName { # [primary_span] # [label] pub span : Span , }
    };
}

EmptyLinkName!();
macro_rules! LinkRequiresName {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_link_requires_name , code = E0459)] pub (crate) struct LinkRequiresName { # [primary_span] # [label] pub span : Span , }
    };
}

LinkRequiresName!()
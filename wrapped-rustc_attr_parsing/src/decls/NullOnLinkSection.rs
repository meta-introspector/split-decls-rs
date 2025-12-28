macro_rules! NullOnLinkSection {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_null_on_link_section , code = E0648)] pub (crate) struct NullOnLinkSection { # [primary_span] pub span : Span , }
    };
}

NullOnLinkSection!()
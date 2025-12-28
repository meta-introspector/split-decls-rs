macro_rules! InvalidLinkModifier {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_invalid_link_modifier)] pub (crate) struct InvalidLinkModifier { # [primary_span] pub span : Span , }
    };
}

InvalidLinkModifier!()
macro_rules! InvalidAttrUnsafe {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_invalid_attr_unsafe)] # [note] pub (crate) struct InvalidAttrUnsafe { # [primary_span] # [label] pub span : Span , pub name : Path , }
    };
}

InvalidAttrUnsafe!();
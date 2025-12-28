macro_rules! ReprIdent {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_repr_ident , code = E0565)] pub (crate) struct ReprIdent { # [primary_span] pub span : Span , }
    };
}

ReprIdent!();
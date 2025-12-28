macro_rules! InvalidReprHintNoParen {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_invalid_repr_hint_no_paren , code = E0552)] pub (crate) struct InvalidReprHintNoParen { # [primary_span] pub span : Span , pub name : Symbol , }
    };
}

InvalidReprHintNoParen!()
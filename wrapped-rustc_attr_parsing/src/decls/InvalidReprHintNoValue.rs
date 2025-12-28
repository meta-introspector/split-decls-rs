macro_rules! InvalidReprHintNoValue {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_invalid_repr_hint_no_value , code = E0552)] pub (crate) struct InvalidReprHintNoValue { # [primary_span] pub span : Span , pub name : Symbol , }
    };
}

InvalidReprHintNoValue!();
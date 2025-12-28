macro_rules! UnrecognizedReprHint {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_unrecognized_repr_hint , code = E0552)] # [help] # [note] pub (crate) struct UnrecognizedReprHint { # [primary_span] pub span : Span , }
    };
}

UnrecognizedReprHint!()
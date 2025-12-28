macro_rules! InvalidPredicate {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_invalid_predicate , code = E0537)] pub (crate) struct InvalidPredicate { # [primary_span] pub span : Span , pub predicate : String , }
    };
}

InvalidPredicate!()
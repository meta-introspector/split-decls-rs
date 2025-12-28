macro_rules! Ok {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_ok)] pub (crate) struct Ok { # [primary_span] pub span : Span , }
    };
}

Ok!()
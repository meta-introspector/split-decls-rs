macro_rules! MissingDepNode {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_missing_depnode)] pub (crate) struct MissingDepNode { # [primary_span] pub span : Span , }
    };
}

MissingDepNode!()
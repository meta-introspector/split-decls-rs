macro_rules! UnrecognizedDepNode {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_unrecognized_depnode)] pub (crate) struct UnrecognizedDepNode { # [primary_span] pub span : Span , pub name : Symbol , }
    };
}

UnrecognizedDepNode!()
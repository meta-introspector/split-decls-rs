macro_rules! UnrecognizedDepNodeLabel {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_unrecognized_depnode_label)] pub (crate) struct UnrecognizedDepNodeLabel < 'a > { # [primary_span] pub span : Span , pub label : & 'a str , }
    };
}

UnrecognizedDepNodeLabel!();
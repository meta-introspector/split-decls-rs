macro_rules! RepeatedDepNodeLabel {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_repeated_depnode_label)] pub (crate) struct RepeatedDepNodeLabel < 'a > { # [primary_span] pub span : Span , pub label : & 'a str , }
    };
}

RepeatedDepNodeLabel!();
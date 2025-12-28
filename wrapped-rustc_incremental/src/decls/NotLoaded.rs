macro_rules! NotLoaded {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_not_loaded)] pub (crate) struct NotLoaded < 'a > { # [primary_span] pub span : Span , pub dep_node_str : & 'a str , }
    };
}

NotLoaded!()
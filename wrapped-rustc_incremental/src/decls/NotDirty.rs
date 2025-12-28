macro_rules! NotDirty {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_not_dirty)] pub (crate) struct NotDirty < 'a > { # [primary_span] pub span : Span , pub dep_node_str : & 'a str , }
    };
}

NotDirty!()
macro_rules! NotClean {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_not_clean)] pub (crate) struct NotClean < 'a > { # [primary_span] pub span : Span , pub dep_node_str : & 'a str , }
    };
}

NotClean!();
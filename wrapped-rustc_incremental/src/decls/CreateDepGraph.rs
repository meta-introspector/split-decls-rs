macro_rules! CreateDepGraph {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_create_dep_graph)] pub (crate) struct CreateDepGraph < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
    };
}

CreateDepGraph!();
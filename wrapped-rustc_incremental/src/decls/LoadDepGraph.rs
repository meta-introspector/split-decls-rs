macro_rules! LoadDepGraph {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_load_dep_graph)] pub (crate) struct LoadDepGraph { pub path : PathBuf , pub err : std :: io :: Error , }
    };
}

LoadDepGraph!();
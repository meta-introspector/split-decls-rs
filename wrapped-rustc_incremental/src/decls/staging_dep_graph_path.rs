macro_rules! staging_dep_graph_path {
    () => {
        # [doc = " Returns the path to a session's staging dependency graph."] # [doc = ""] # [doc = " On the difference between dep-graph and staging dep-graph,"] # [doc = " see `build_dep_graph`."] pub (crate) fn staging_dep_graph_path (sess : & Session) -> PathBuf { in_incr_comp_dir_sess (sess , STAGING_DEP_GRAPH_FILENAME) }
    };
}

staging_dep_graph_path!();
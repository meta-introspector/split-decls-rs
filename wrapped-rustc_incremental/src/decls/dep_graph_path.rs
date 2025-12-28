macro_rules! dep_graph_path {
    () => {
        # [doc = " Returns the path to a session's dependency graph."] pub (crate) fn dep_graph_path (sess : & Session) -> PathBuf { in_incr_comp_dir_sess (sess , DEP_GRAPH_FILENAME) }
    };
}

dep_graph_path!();
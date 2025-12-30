// Generated macro for dep_graph_path (function)
macro_rules! Depcrate_persist_fsdep_graph_path {
() => {
// Module: crate::persist::fs
// Provides: {"dep_graph_path"}
// Dependencies: {}
# [doc = " Returns the path to a session's dependency graph."] pub (crate) fn dep_graph_path (sess : & Session) -> PathBuf { in_incr_comp_dir_sess (sess , DEP_GRAPH_FILENAME) }
};
}

// Generated macro for query_cache_path (function)
macro_rules! Depcrate_persist_fsquery_cache_path {
() => {
// Module: crate::persist::fs
// Provides: {"query_cache_path"}
// Dependencies: {}
# [doc = " Returns the path to a session's query cache."] pub (crate) fn query_cache_path (sess : & Session) -> PathBuf { in_incr_comp_dir_sess (sess , QUERY_CACHE_FILENAME) }
};
}

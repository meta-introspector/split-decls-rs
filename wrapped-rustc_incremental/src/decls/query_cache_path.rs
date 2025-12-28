macro_rules! query_cache_path {
    () => {
        # [doc = " Returns the path to a session's query cache."] pub (crate) fn query_cache_path (sess : & Session) -> PathBuf { in_incr_comp_dir_sess (sess , QUERY_CACHE_FILENAME) }
    };
}

query_cache_path!();
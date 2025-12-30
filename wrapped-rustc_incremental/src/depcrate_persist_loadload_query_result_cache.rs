// Generated macro for load_query_result_cache (function)
macro_rules! Depcrate_persist_loadload_query_result_cache {
() => {
// Module: crate::persist::load
// Provides: {"load_query_result_cache"}
// Dependencies: {}
# [doc = " Attempts to load the query result cache from disk"] # [doc = ""] # [doc = " If we are not in incremental compilation mode, returns `None`."] # [doc = " Otherwise, tries to load the query result cache from disk,"] # [doc = " creating an empty cache if it could not be loaded."] pub fn load_query_result_cache (sess : & Session) -> Option < OnDiskCache > { if sess . opts . incremental . is_none () { return None ; } let _prof_timer = sess . prof . generic_activity ("incr_comp_load_query_result_cache") ; let path = query_cache_path (sess) ; match load_data (& path , sess) { LoadResult :: Ok { data : (bytes , start_pos) } => { let cache = OnDiskCache :: new (sess , bytes , start_pos) . unwrap_or_else (| () | { sess . dcx () . emit_warn (errors :: CorruptFile { path : & path }) ; OnDiskCache :: new_empty () }) ; Some (cache) } _ => Some (OnDiskCache :: new_empty ()) , } }
};
}

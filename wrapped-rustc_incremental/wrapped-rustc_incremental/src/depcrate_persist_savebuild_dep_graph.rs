// Generated macro for build_dep_graph (function)
macro_rules! Depcrate_persist_savebuild_dep_graph {
() => {
// Module: crate::persist::save
// Provides: {"build_dep_graph"}
// Dependencies: {}
# [doc = " Builds the dependency graph."] # [doc = ""] # [doc = " This function creates the *staging dep-graph*. When the dep-graph is modified by a query"] # [doc = " execution, the new dependency information is not kept in memory but directly"] # [doc = " output to this file. `save_dep_graph` then finalizes the staging dep-graph"] # [doc = " and moves it to the permanent dep-graph path"] pub (crate) fn build_dep_graph (sess : & Session , prev_graph : Arc < SerializedDepGraph > , prev_work_products : WorkProductMap ,) -> Option < DepGraph > { if sess . opts . incremental . is_none () { return None ; } let path_buf = staging_dep_graph_path (sess) ; let mut encoder = match FileEncoder :: new (& path_buf) { Ok (encoder) => encoder , Err (err) => { sess . dcx () . emit_err (errors :: CreateDepGraph { path : & path_buf , err }) ; return None ; } } ; file_format :: write_file_header (& mut encoder , sess) ; sess . opts . dep_tracking_hash (false) . encode (& mut encoder) ; Some (DepGraph :: new (sess , prev_graph , prev_work_products , encoder)) }
};
}

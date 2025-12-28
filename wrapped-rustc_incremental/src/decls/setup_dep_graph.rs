macro_rules! setup_dep_graph {
    () => {
        # [doc = " Setups the dependency graph by loading an existing graph from disk and set up streaming of a"] # [doc = " new graph to an incremental session directory."] pub fn setup_dep_graph (sess : & Session , crate_name : Symbol , deps : & DepsType) -> DepGraph { prepare_session_directory (sess , crate_name) ; let res = sess . opts . build_dep_graph () . then (| | load_dep_graph (sess , deps)) ; if sess . opts . incremental . is_some () { sess . time ("incr_comp_garbage_collect_session_directories" , | | { if let Err (e) = garbage_collect_session_directories (sess) { warn ! ("Error while trying to garbage collect incremental \
                     compilation cache directory: {}" , e) ; } }) ; } res . and_then (| result | { let (prev_graph , prev_work_products) = result . open (sess) ; build_dep_graph (sess , prev_graph , prev_work_products) }) . unwrap_or_else (DepGraph :: new_disabled) }
    };
}

setup_dep_graph!()
macro_rules! deps {
    () => {
        MoveDepGraph!();
    };
}

macro_rules! save_dep_graph {
    () => {
        deps!();
        # [doc = " Saves and writes the [`DepGraph`] to the file system."] # [doc = ""] # [doc = " This function saves both the dep-graph and the query result cache,"] # [doc = " and drops the result cache."] # [doc = ""] # [doc = " This function should only run after all queries have completed."] # [doc = " Trying to execute a query afterwards would attempt to read the result cache we just dropped."] pub (crate) fn save_dep_graph (tcx : TyCtxt < '_ >) { debug ! ("save_dep_graph()") ; tcx . dep_graph . with_ignore (| | { let sess = tcx . sess ; if sess . opts . incremental . is_none () { return ; } if sess . dcx () . has_errors_or_delayed_bugs () . is_some () { return ; } let query_cache_path = query_cache_path (sess) ; let dep_graph_path = dep_graph_path (sess) ; let staging_dep_graph_path = staging_dep_graph_path (sess) ; sess . time ("assert_dep_graph" , | | assert_dep_graph (tcx)) ; sess . time ("check_dirty_clean" , | | dirty_clean :: check_dirty_clean_annotations (tcx)) ; join (move | | { sess . time ("incr_comp_persist_dep_graph" , | | { if let Err (err) = fs :: rename (& staging_dep_graph_path , & dep_graph_path) { sess . dcx () . emit_err (errors :: MoveDepGraph { from : & staging_dep_graph_path , to : & dep_graph_path , err , }) ; } }) ; } , move | | { sess . time ("incr_comp_persist_result_cache" , | | { if let Some (odc) = & tcx . query_system . on_disk_cache { odc . drop_serialized_data (tcx) ; } file_format :: save_in (sess , query_cache_path , "query cache" , | e | { encode_query_cache (tcx , e) }) ; }) ; } ,) ; }) }
    };
}

save_dep_graph!()
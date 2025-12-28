macro_rules! setup_callbacks {
    () => {
        # [doc = " Sets up the callbacks in prior crates which we want to refer to the"] # [doc = " TyCtxt in."] pub fn setup_callbacks () { rustc_span :: SPAN_TRACK . swap (& (track_span_parent as fn (_))) ; rustc_hir :: def_id :: DEF_ID_DEBUG . swap (& (def_id_debug as fn (_ , & mut fmt :: Formatter < '_ >) -> _)) ; rustc_query_system :: dep_graph :: dep_node :: DEP_KIND_DEBUG . swap (& (dep_kind_debug as fn (_ , & mut fmt :: Formatter < '_ >) -> _)) ; rustc_query_system :: dep_graph :: dep_node :: DEP_NODE_DEBUG . swap (& (dep_node_debug as fn (_ , & mut fmt :: Formatter < '_ >) -> _)) ; TRACK_DIAGNOSTIC . swap (& (track_diagnostic as _)) ; }
    };
}

setup_callbacks!();
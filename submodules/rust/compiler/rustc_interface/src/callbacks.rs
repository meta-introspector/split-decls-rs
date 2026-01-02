mkuse!{use std :: fmt ;}
mkuse!{use rustc_errors :: { DiagInner , TRACK_DIAGNOSTIC } ;}
mkuse!{use rustc_middle :: dep_graph :: { DepNodeExt , TaskDepsRef } ;}
mkuse!{use rustc_middle :: ty :: tls ;}
mkuse!{use rustc_query_impl :: QueryCtxt ;}
mkuse!{use rustc_query_system :: dep_graph :: dep_node :: default_dep_kind_debug ;}
mkuse!{use rustc_query_system :: dep_graph :: { DepContext , DepKind , DepNode } ;}

macro_rules! track_span_parent_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function track_span_parent in module {}", module_path!());
    };
}

mkfn!{
    track_span_parent_introspect!();
    fn track_span_parent (def_id : rustc_span :: def_id :: LocalDefId) { tls :: with_context_opt (| icx | { if let Some (icx) = icx { let tracks_deps = match icx . task_deps { TaskDepsRef :: Allow (..) => true , TaskDepsRef :: EvalAlways | TaskDepsRef :: Ignore | TaskDepsRef :: Forbid => false , } ; if tracks_deps { let _span = icx . tcx . source_span (def_id) ; debug_assert_eq ! (_span . data_untracked () . parent , None) ; } } }) }
}

macro_rules! track_diagnostic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function track_diagnostic in module {}", module_path!());
    };
}

mkfn!{
    track_diagnostic_introspect!();
    # [doc = " This is a callback from `rustc_errors` as it cannot access the implicit state"] # [doc = " in `rustc_middle` otherwise. It is used when diagnostic messages are"] # [doc = " emitted and stores them in the current query, if there is one."] fn track_diagnostic < R > (diagnostic : DiagInner , f : & mut dyn FnMut (DiagInner) -> R) -> R { tls :: with_context_opt (| icx | { if let Some (icx) = icx { icx . tcx . dep_graph . record_diagnostic (QueryCtxt :: new (icx . tcx) , & diagnostic) ; let icx = tls :: ImplicitCtxt { task_deps : TaskDepsRef :: Ignore , .. icx . clone () } ; tls :: enter_context (& icx , move | | (* f) (diagnostic)) } else { (* f) (diagnostic) } }) }
}

macro_rules! def_id_debug_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function def_id_debug in module {}", module_path!());
    };
}

mkfn!{
    def_id_debug_introspect!();
    # [doc = " This is a callback from `rustc_hir` as it cannot access the implicit state"] # [doc = " in `rustc_middle` otherwise."] fn def_id_debug (def_id : rustc_hir :: def_id :: DefId , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "DefId({}:{}" , def_id . krate , def_id . index . index ()) ? ; tls :: with_opt (| opt_tcx | { if let Some (tcx) = opt_tcx { write ! (f , " ~ {}" , tcx . def_path_debug_str (def_id)) ? ; } Ok (()) }) ? ; write ! (f , ")") }
}

macro_rules! dep_kind_debug_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dep_kind_debug in module {}", module_path!());
    };
}

mkfn!{
    dep_kind_debug_introspect!();
    # [doc = " This is a callback from `rustc_query_system` as it cannot access the implicit state"] # [doc = " in `rustc_middle` otherwise."] pub fn dep_kind_debug (kind : DepKind , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { tls :: with_opt (| opt_tcx | { if let Some (tcx) = opt_tcx { write ! (f , "{}" , tcx . dep_kind_info (kind) . name) } else { default_dep_kind_debug (kind , f) } }) }
}

macro_rules! dep_node_debug_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dep_node_debug in module {}", module_path!());
    };
}

mkfn!{
    dep_node_debug_introspect!();
    # [doc = " This is a callback from `rustc_query_system` as it cannot access the implicit state"] # [doc = " in `rustc_middle` otherwise."] pub fn dep_node_debug (node : DepNode , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{:?}(" , node . kind) ? ; tls :: with_opt (| opt_tcx | { if let Some (tcx) = opt_tcx { if let Some (def_id) = node . extract_def_id (tcx) { write ! (f , "{}" , tcx . def_path_debug_str (def_id)) ? ; } else if let Some (ref s) = tcx . dep_graph . dep_node_debug_str (node) { write ! (f , "{s}") ? ; } else { write ! (f , "{}" , node . hash) ? ; } } else { write ! (f , "{}" , node . hash) ? ; } Ok (()) }) ? ; write ! (f , ")") }
}

macro_rules! setup_callbacks_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function setup_callbacks in module {}", module_path!());
    };
}

mkfn!{
    setup_callbacks_introspect!();
    # [doc = " Sets up the callbacks in prior crates which we want to refer to the"] # [doc = " TyCtxt in."] pub fn setup_callbacks () { rustc_span :: SPAN_TRACK . swap (& (track_span_parent as fn (_))) ; rustc_hir :: def_id :: DEF_ID_DEBUG . swap (& (def_id_debug as fn (_ , & mut fmt :: Formatter < '_ >) -> _)) ; rustc_query_system :: dep_graph :: dep_node :: DEP_KIND_DEBUG . swap (& (dep_kind_debug as fn (_ , & mut fmt :: Formatter < '_ >) -> _)) ; rustc_query_system :: dep_graph :: dep_node :: DEP_NODE_DEBUG . swap (& (dep_node_debug as fn (_ , & mut fmt :: Formatter < '_ >) -> _)) ; TRACK_DIAGNOSTIC . swap (& (track_diagnostic as _)) ; }
}
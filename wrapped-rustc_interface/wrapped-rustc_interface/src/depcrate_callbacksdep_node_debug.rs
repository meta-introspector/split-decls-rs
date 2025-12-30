// Generated macro for dep_node_debug (function)
macro_rules! Depcrate_callbacksdep_node_debug {
() => {
// Module: crate::callbacks
// Provides: {"dep_node_debug"}
// Dependencies: {}
# [doc = " This is a callback from `rustc_query_system` as it cannot access the implicit state"] # [doc = " in `rustc_middle` otherwise."] pub fn dep_node_debug (node : DepNode , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{:?}(" , node . kind) ? ; tls :: with_opt (| opt_tcx | { if let Some (tcx) = opt_tcx { if let Some (def_id) = node . extract_def_id (tcx) { write ! (f , "{}" , tcx . def_path_debug_str (def_id)) ? ; } else if let Some (ref s) = tcx . dep_graph . dep_node_debug_str (node) { write ! (f , "{s}") ? ; } else { write ! (f , "{}" , node . hash) ? ; } } else { write ! (f , "{}" , node . hash) ? ; } Ok (()) }) ? ; write ! (f , ")") }
};
}

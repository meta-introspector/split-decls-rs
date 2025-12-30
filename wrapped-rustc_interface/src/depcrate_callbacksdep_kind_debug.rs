// Generated macro for dep_kind_debug (function)
macro_rules! Depcrate_callbacksdep_kind_debug {
() => {
// Module: crate::callbacks
// Provides: {"dep_kind_debug"}
// Dependencies: {}
# [doc = " This is a callback from `rustc_query_system` as it cannot access the implicit state"] # [doc = " in `rustc_middle` otherwise."] pub fn dep_kind_debug (kind : DepKind , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { tls :: with_opt (| opt_tcx | { if let Some (tcx) = opt_tcx { write ! (f , "{}" , tcx . dep_kind_info (kind) . name) } else { default_dep_kind_debug (kind , f) } }) }
};
}

// Generated macro for def_id_debug (function)
macro_rules! Depcrate_callbacksdef_id_debug {
() => {
// Module: crate::callbacks
// Provides: {"def_id_debug"}
// Dependencies: {}
# [doc = " This is a callback from `rustc_hir` as it cannot access the implicit state"] # [doc = " in `rustc_middle` otherwise."] fn def_id_debug (def_id : rustc_hir :: def_id :: DefId , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "DefId({}:{}" , def_id . krate , def_id . index . index ()) ? ; tls :: with_opt (| opt_tcx | { if let Some (tcx) = opt_tcx { write ! (f , " ~ {}" , tcx . def_path_debug_str (def_id)) ? ; } Ok (()) }) ? ; write ! (f , ")") }
};
}

// Generated macro for proc_macro_decls_static (function)
macro_rules! Depcrate_proc_macro_declsproc_macro_decls_static {
() => {
// Module: crate::proc_macro_decls
// Provides: {"proc_macro_decls_static"}
// Dependencies: {}
fn proc_macro_decls_static (tcx : TyCtxt < '_ > , () : ()) -> Option < LocalDefId > { let mut decls = None ; for id in tcx . hir_free_items () { let attrs = tcx . hir_attrs (id . hir_id ()) ; if attr :: contains_name (attrs , sym :: rustc_proc_macro_decls) { decls = Some (id . owner_id . def_id) ; } } decls }
};
}

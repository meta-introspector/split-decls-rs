macro_rules! proc_macro_decls_static {
    () => {
        fn proc_macro_decls_static (tcx : TyCtxt < '_ > , () : ()) -> Option < LocalDefId > { let mut decls = None ; for id in tcx . hir_free_items () { let attrs = tcx . hir_attrs (id . hir_id ()) ; if attr :: contains_name (attrs , sym :: rustc_proc_macro_decls) { decls = Some (id . owner_id . def_id) ; } } decls }
    };
}

proc_macro_decls_static!();
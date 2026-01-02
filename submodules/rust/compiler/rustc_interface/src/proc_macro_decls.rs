mkuse!{use rustc_ast :: attr ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_span :: sym ;}

macro_rules! proc_macro_decls_static_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function proc_macro_decls_static in module {}", module_path!());
    };
}

mkfn!{
    proc_macro_decls_static_introspect!();
    fn proc_macro_decls_static (tcx : TyCtxt < '_ > , () : ()) -> Option < LocalDefId > { let mut decls = None ; for id in tcx . hir_free_items () { let attrs = tcx . hir_attrs (id . hir_id ()) ; if attr :: contains_name (attrs , sym :: rustc_proc_macro_decls) { decls = Some (id . owner_id . def_id) ; } } decls }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { proc_macro_decls_static , .. * providers } ; }
}
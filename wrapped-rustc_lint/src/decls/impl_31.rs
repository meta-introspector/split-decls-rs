macro_rules! deps {
    () => {
        LateContext!();
        BuiltinMissingDoc!();
        MissingDoc!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl MissingDoc { fn check_missing_docs_attrs (& self , cx : & LateContext < '_ > , def_id : LocalDefId , article : & 'static str , desc : & 'static str ,) { if def_id != CRATE_DEF_ID && ! cx . effective_visibilities . is_exported (def_id) { return ; } let attrs = cx . tcx . hir_attrs (cx . tcx . local_def_id_to_hir_id (def_id)) ; let has_doc = attrs . iter () . any (has_doc) ; if ! has_doc { cx . emit_span_lint (MISSING_DOCS , cx . tcx . def_span (def_id) , BuiltinMissingDoc { article , desc } ,) ; } } }
    };
}

impl_31!()
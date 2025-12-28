macro_rules! mk_lint {
    () => {
        fn mk_lint (tcx : TyCtxt < '_ > , diag : & mut Diag < '_ , () > , type_def_id : DefId , impl_def_id : DefId , orig_fields : FxHashMap < Symbol , & hir :: FieldDef < '_ > > , fields : & [hir :: ExprField < '_ >] ,) { diag . primary_message ("`Default` impl doesn't use the declared default field values") ; let mut removed_all_fields = true ; for field in fields { if orig_fields . get (& field . ident . name) . and_then (| f | f . default) . is_some () { diag . span_label (field . expr . span , "this field has a default value") ; } else { removed_all_fields = false ; } } if removed_all_fields { let msg = "to avoid divergence in behavior between `Struct { .. }` and \
                   `<Struct as Default>::default()`, derive the `Default`" ; if let Some (hir :: Node :: Item (impl_)) = tcx . hir_get_if_local (impl_def_id) { diag . multipart_suggestion_verbose (msg , vec ! [(tcx . def_span (type_def_id) . shrink_to_lo () , "#[derive(Default)] " . to_string ()) , (impl_ . span , String :: new ()) ,] , Applicability :: MachineApplicable ,) ; } else { diag . help (msg) ; } } else { let msg = "use the default values in the `impl` with `Struct { mandatory_field, .. }` to \
                   avoid them diverging over time" ; diag . help (msg) ; } }
    };
}

mk_lint!()
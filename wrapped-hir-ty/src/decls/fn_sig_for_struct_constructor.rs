macro_rules! deps {
    () => {
        FnSig!();
        EarlyBinder!();
        HirDatabase!();
        PolyFnSig!();
        FnAbi!();
        Binder!();
    };
}

macro_rules! fn_sig_for_struct_constructor {
    () => {
        deps!();
        fn fn_sig_for_struct_constructor < 'db > (db : & 'db dyn HirDatabase , def : StructId ,) -> EarlyBinder < 'db , PolyFnSig < 'db > > { let field_tys = db . field_types (def . into ()) ; let params = field_tys . iter () . map (| (_ , ty) | ty . skip_binder ()) ; let ret = type_for_adt (db , def . into ()) . skip_binder () ; let inputs_and_output = Tys :: new_from_iter (DbInterner :: new_with (db , None , None) , params . chain (Some (ret))) ; EarlyBinder :: bind (Binder :: dummy (FnSig { abi : FnAbi :: RustCall , c_variadic : false , safety : Safety :: Safe , inputs_and_output , })) }
    };
}

fn_sig_for_struct_constructor!()
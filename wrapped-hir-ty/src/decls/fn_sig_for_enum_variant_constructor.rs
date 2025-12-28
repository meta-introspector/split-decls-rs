macro_rules! deps {
    () => {
        EarlyBinder!();
        PolyFnSig!();
        FnAbi!();
        HirDatabase!();
        FnSig!();
        Binder!();
    };
}

macro_rules! fn_sig_for_enum_variant_constructor {
    () => {
        deps!();
        fn fn_sig_for_enum_variant_constructor < 'db > (db : & 'db dyn HirDatabase , def : EnumVariantId ,) -> EarlyBinder < 'db , PolyFnSig < 'db > > { let field_tys = db . field_types (def . into ()) ; let params = field_tys . iter () . map (| (_ , ty) | ty . skip_binder ()) ; let parent = def . lookup (db) . parent ; let ret = type_for_adt (db , parent . into ()) . skip_binder () ; let inputs_and_output = Tys :: new_from_iter (DbInterner :: new_with (db , None , None) , params . chain (Some (ret))) ; EarlyBinder :: bind (Binder :: dummy (FnSig { abi : FnAbi :: RustCall , c_variadic : false , safety : Safety :: Safe , inputs_and_output , })) }
    };
}

fn_sig_for_enum_variant_constructor!();
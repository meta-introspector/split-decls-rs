macro_rules! deps {
    () => {
        HirDatabase!();
        EarlyBinder!();
    };
}

macro_rules! type_for_enum_variant_constructor {
    () => {
        deps!();
        # [doc = " Build the type of a tuple enum variant constructor."] fn type_for_enum_variant_constructor < 'db > (db : & 'db dyn HirDatabase , def : EnumVariantId ,) -> Option < EarlyBinder < 'db , Ty < 'db > > > { let struct_data = def . fields (db) ; match struct_data . shape { FieldsShape :: Record => None , FieldsShape :: Unit => Some (type_for_adt (db , def . loc (db) . parent . into ())) , FieldsShape :: Tuple => { let interner = DbInterner :: new_with (db , None , None) ; Some (EarlyBinder :: bind (Ty :: new_fn_def (interner , CallableDefId :: EnumVariantId (def) . into () , GenericArgs :: identity_for_item (interner , def . loc (db) . parent . into ()) ,))) } } }
    };
}

type_for_enum_variant_constructor!();
macro_rules! deps {
    () => {
        EarlyBinder!();
        HirDatabase!();
    };
}

macro_rules! type_for_struct_constructor {
    () => {
        deps!();
        # [doc = " Build the type of a tuple struct constructor."] fn type_for_struct_constructor < 'db > (db : & 'db dyn HirDatabase , def : StructId ,) -> Option < EarlyBinder < 'db , Ty < 'db > > > { let struct_data = def . fields (db) ; match struct_data . shape { FieldsShape :: Record => None , FieldsShape :: Unit => Some (type_for_adt (db , def . into ())) , FieldsShape :: Tuple => { let interner = DbInterner :: new_with (db , None , None) ; Some (EarlyBinder :: bind (Ty :: new_fn_def (interner , CallableDefId :: StructId (def) . into () , GenericArgs :: identity_for_item (interner , def . into ()) ,))) } } }
    };
}

type_for_struct_constructor!()
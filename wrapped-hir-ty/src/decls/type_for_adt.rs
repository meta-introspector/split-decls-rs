macro_rules! deps {
    () => {
        HirDatabase!();
        EarlyBinder!();
    };
}

macro_rules! type_for_adt {
    () => {
        deps!();
        fn type_for_adt < 'db > (db : & 'db dyn HirDatabase , adt : AdtId) -> EarlyBinder < 'db , Ty < 'db > > { let interner = DbInterner :: new_with (db , None , None) ; let args = GenericArgs :: identity_for_item (interner , adt . into ()) ; let ty = Ty :: new_adt (interner , adt , args) ; EarlyBinder :: bind (ty) }
    };
}

type_for_adt!()
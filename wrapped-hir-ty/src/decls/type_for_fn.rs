macro_rules! deps {
    () => {
        HirDatabase!();
        EarlyBinder!();
    };
}

macro_rules! type_for_fn {
    () => {
        deps!();
        # [doc = " Build the declared type of a function. This should not need to look at the"] # [doc = " function body."] fn type_for_fn < 'db > (db : & 'db dyn HirDatabase , def : FunctionId) -> EarlyBinder < 'db , Ty < 'db > > { let interner = DbInterner :: new_with (db , None , None) ; EarlyBinder :: bind (Ty :: new_fn_def (interner , CallableDefId :: FunctionId (def) . into () , GenericArgs :: identity_for_item (interner , def . into ()) ,)) }
    };
}

type_for_fn!()
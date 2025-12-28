macro_rules! deps {
    () => {
        HirDatabase!();
        EarlyBinder!();
        Diagnostics!();
    };
}

macro_rules! type_for_type_alias_with_diagnostics_cycle_result {
    () => {
        deps!();
        pub (crate) fn type_for_type_alias_with_diagnostics_cycle_result < 'db > (db : & 'db dyn HirDatabase , _adt : TypeAliasId ,) -> (EarlyBinder < 'db , Ty < 'db > > , Diagnostics) { (EarlyBinder :: bind (Ty :: new_error (DbInterner :: new_with (db , None , None) , ErrorGuaranteed)) , None) }
    };
}

type_for_type_alias_with_diagnostics_cycle_result!();
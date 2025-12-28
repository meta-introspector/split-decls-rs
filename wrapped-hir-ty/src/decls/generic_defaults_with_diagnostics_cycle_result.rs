macro_rules! deps {
    () => {
        Diagnostics!();
        HirDatabase!();
        GenericDefaults!();
    };
}

macro_rules! generic_defaults_with_diagnostics_cycle_result {
    () => {
        deps!();
        pub (crate) fn generic_defaults_with_diagnostics_cycle_result (_db : & dyn HirDatabase , _def : GenericDefId ,) -> (GenericDefaults < '_ > , Diagnostics) { (GenericDefaults (None) , None) }
    };
}

generic_defaults_with_diagnostics_cycle_result!();
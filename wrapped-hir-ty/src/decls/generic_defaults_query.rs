macro_rules! deps {
    () => {
        GenericDefaults!();
        HirDatabase!();
    };
}

macro_rules! generic_defaults_query {
    () => {
        deps!();
        pub (crate) fn generic_defaults_query (db : & dyn HirDatabase , def : GenericDefId ,) -> GenericDefaults < '_ > { db . generic_defaults_with_diagnostics (def) . 0 }
    };
}

generic_defaults_query!()
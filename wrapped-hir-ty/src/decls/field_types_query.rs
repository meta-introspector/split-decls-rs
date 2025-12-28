macro_rules! deps {
    () => {
        EarlyBinder!();
        HirDatabase!();
    };
}

macro_rules! field_types_query {
    () => {
        deps!();
        pub (crate) fn field_types_query < 'db > (db : & 'db dyn HirDatabase , variant_id : VariantId ,) -> Arc < ArenaMap < LocalFieldId , EarlyBinder < 'db , Ty < 'db > > > > { db . field_types_with_diagnostics (variant_id) . 0 }
    };
}

field_types_query!();
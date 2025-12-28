macro_rules! deps {
    () => {
        HirDatabase!();
        EarlyBinder!();
    };
}

macro_rules! type_alias_bounds {
    () => {
        deps!();
        # [inline] pub (crate) fn type_alias_bounds < 'db > (db : & 'db dyn HirDatabase , type_alias : TypeAliasId ,) -> EarlyBinder < 'db , & 'db [Clause < 'db >] > { type_alias_bounds_with_diagnostics (db , type_alias) . 0 . as_ref () . map_bound (| it | & * * it) }
    };
}

type_alias_bounds!()
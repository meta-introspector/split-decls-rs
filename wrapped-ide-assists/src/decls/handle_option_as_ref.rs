macro_rules! deps {
    () => {
        ReferenceConversionType!();
    };
}

macro_rules! handle_option_as_ref {
    () => {
        deps!();
        fn handle_option_as_ref (ty : & hir :: Type < '_ > , db : & dyn HirDatabase , famous_defs : & FamousDefs < '_ , '_ > ,) -> Option < (ReferenceConversionType , bool) > { if ty . as_adt () == famous_defs . core_option_Option () ? . ty (db) . as_adt () { Some ((ReferenceConversionType :: Option , false)) } else { None } }
    };
}

handle_option_as_ref!()
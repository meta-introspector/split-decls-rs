macro_rules! deps {
    () => {
        ReferenceConversionType!();
    };
}

macro_rules! handle_result_as_ref {
    () => {
        deps!();
        fn handle_result_as_ref (ty : & hir :: Type < '_ > , db : & dyn HirDatabase , famous_defs : & FamousDefs < '_ , '_ > ,) -> Option < (ReferenceConversionType , bool) > { if ty . as_adt () == famous_defs . core_result_Result () ? . ty (db) . as_adt () { Some ((ReferenceConversionType :: Result , false)) } else { None } }
    };
}

handle_result_as_ref!();
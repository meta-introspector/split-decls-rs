macro_rules! deps {
    () => {
        ReferenceConversionType!();
    };
}

macro_rules! handle_dereferenced {
    () => {
        deps!();
        fn handle_dereferenced (ty : & hir :: Type < '_ > , db : & dyn HirDatabase , famous_defs : & FamousDefs < '_ , '_ > ,) -> Option < (ReferenceConversionType , bool) > { let type_argument = ty . type_arguments () . next () ? ; ty . impls_trait (db , famous_defs . core_convert_AsRef () ? , slice :: from_ref (& type_argument)) . then_some ((ReferenceConversionType :: Dereferenced , could_deref_to_target (ty , & type_argument , db) ,)) }
    };
}

handle_dereferenced!()
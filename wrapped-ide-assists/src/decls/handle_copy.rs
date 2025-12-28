macro_rules! deps {
    () => {
        ReferenceConversionType!();
    };
}

macro_rules! handle_copy {
    () => {
        deps!();
        fn handle_copy (ty : & hir :: Type < '_ > , db : & dyn HirDatabase ,) -> Option < (ReferenceConversionType , bool) > { ty . is_copy (db) . then_some ((ReferenceConversionType :: Copy , true)) }
    };
}

handle_copy!()
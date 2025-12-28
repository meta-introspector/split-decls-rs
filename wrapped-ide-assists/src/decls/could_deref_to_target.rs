macro_rules! could_deref_to_target {
    () => {
        fn could_deref_to_target (ty : & hir :: Type < '_ > , target : & hir :: Type < '_ > , db : & dyn HirDatabase) -> bool { let ty_ref = ty . add_reference (hir :: Mutability :: Shared) ; let target_ref = target . add_reference (hir :: Mutability :: Shared) ; ty_ref . could_coerce_to (db , & target_ref) }
    };
}

could_deref_to_target!();
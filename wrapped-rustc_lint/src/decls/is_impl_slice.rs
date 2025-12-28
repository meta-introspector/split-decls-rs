macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! is_impl_slice {
    () => {
        deps!();
        fn is_impl_slice (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { if let Some (method_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) && let Some (impl_id) = cx . tcx . impl_of_assoc (method_id) { return cx . tcx . type_of (impl_id) . skip_binder () . is_slice () ; } false }
    };
}

is_impl_slice!();
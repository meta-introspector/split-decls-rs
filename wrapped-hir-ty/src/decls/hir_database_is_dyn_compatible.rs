macro_rules! deps {
    () => {
        HirDatabase!();
    };
}

macro_rules! hir_database_is_dyn_compatible {
    () => {
        deps!();
        # [test] fn hir_database_is_dyn_compatible () { fn _assert_dyn_compatible (_ : & dyn HirDatabase) { } }
    };
}

hir_database_is_dyn_compatible!()
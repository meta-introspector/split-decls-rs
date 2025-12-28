macro_rules! has_test_function_or_multiple_test_submodules {
    () => {
        fn has_test_function_or_multiple_test_submodules (sema : & Semantics < '_ , RootDatabase > , module : & hir :: Module , consider_exported_main : bool ,) -> bool { let mut number_of_test_submodules = 0 ; for item in module . declarations (sema . db) { match item { hir :: ModuleDef :: Function (f) => { if has_test_related_attribute (& f . attrs (sema . db)) { return true ; } if consider_exported_main && f . exported_main (sema . db) { return true ; } } hir :: ModuleDef :: Module (submodule) => { if has_test_function_or_multiple_test_submodules (sema , & submodule , consider_exported_main ,) { number_of_test_submodules += 1 ; } } _ => () , } } number_of_test_submodules > 1 }
    };
}

has_test_function_or_multiple_test_submodules!()
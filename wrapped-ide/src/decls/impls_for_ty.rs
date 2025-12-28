macro_rules! deps {
    () => {
        NavigationTarget!();
    };
}

macro_rules! impls_for_ty {
    () => {
        deps!();
        fn impls_for_ty (sema : & Semantics < '_ , RootDatabase > , ty : hir :: Type < '_ >) -> Vec < NavigationTarget > { Impl :: all_for_type (sema . db , ty) . into_iter () . filter_map (| imp | imp . try_to_nav (sema)) . flatten () . collect () }
    };
}

impls_for_ty!()
macro_rules! deps {
    () => {
        NavigationTarget!();
    };
}

macro_rules! impls_for_trait {
    () => {
        deps!();
        fn impls_for_trait (sema : & Semantics < '_ , RootDatabase > , trait_ : hir :: Trait ,) -> Vec < NavigationTarget > { Impl :: all_for_trait (sema . db , trait_) . into_iter () . filter_map (| imp | imp . try_to_nav (sema)) . flatten () . collect () }
    };
}

impls_for_trait!()
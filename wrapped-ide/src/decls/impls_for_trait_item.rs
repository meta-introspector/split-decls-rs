macro_rules! deps {
    () => {
        NavigationTarget!();
    };
}

macro_rules! impls_for_trait_item {
    () => {
        deps!();
        fn impls_for_trait_item (sema : & Semantics < '_ , RootDatabase > , trait_ : hir :: Trait , fun_name : hir :: Name ,) -> Vec < NavigationTarget > { Impl :: all_for_trait (sema . db , trait_) . into_iter () . filter_map (| imp | { let item = imp . items (sema . db) . iter () . find_map (| itm | { let itm_name = itm . name (sema . db) ? ; (itm_name == fun_name) . then_some (* itm) }) ? ; item . try_to_nav (sema) }) . flatten () . collect () }
    };
}

impls_for_trait_item!();
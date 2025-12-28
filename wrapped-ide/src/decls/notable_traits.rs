macro_rules! notable_traits {
    () => {
        fn notable_traits < 'db > (db : & 'db RootDatabase , ty : & hir :: Type < 'db > ,) -> Vec < (hir :: Trait , Vec < (Option < hir :: Type < 'db > > , hir :: Name) >) > { if ty . is_unknown () { return Vec :: new () ; } ty . krate (db) . notable_traits_in_deps (db) . filter_map (move | & trait_ | { let trait_ = trait_ . into () ; ty . impls_trait (db , trait_ , & []) . then (| | { (trait_ , trait_ . items (db) . into_iter () . filter_map (hir :: AssocItem :: as_type_alias) . map (| alias | { (ty . normalize_trait_assoc_type (db , & [] , alias) , alias . name (db)) }) . collect :: < Vec < _ > > () ,) }) }) . sorted_by_cached_key (| (trait_ , _) | trait_ . name (db)) . collect :: < Vec < _ > > () }
    };
}

notable_traits!();
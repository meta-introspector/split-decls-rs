macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! has_unstable_into_iter_predicate {
    () => {
        deps!();
        fn has_unstable_into_iter_predicate < 'tcx > (cx : & LateContext < 'tcx > , callee_def_id : DefId , generic_args : GenericArgsRef < 'tcx > ,) -> bool { let Some (into_iterator_def_id) = cx . tcx . get_diagnostic_item (sym :: IntoIterator) else { return false ; } ; let Some (into_iter_fn_def_id) = cx . tcx . lang_items () . into_iter_fn () else { return false ; } ; let predicates = cx . tcx . predicates_of (callee_def_id) . instantiate (cx . tcx , generic_args) ; for (predicate , _) in predicates { let Some (trait_pred) = predicate . as_trait_clause () else { continue ; } ; if trait_pred . def_id () != into_iterator_def_id || trait_pred . polarity () != PredicatePolarity :: Positive { continue ; } let into_iter_fn_args = cx . tcx . instantiate_bound_regions_with_erased (trait_pred) . trait_ref . args ; let Ok (Some (instance)) = ty :: Instance :: try_resolve (cx . tcx , cx . typing_env () , into_iter_fn_def_id , into_iter_fn_args ,) else { continue ; } ; if cx . tcx . has_attr (instance . def_id () , sym :: rustc_lint_query_instability) { return true ; } } false }
    };
}

has_unstable_into_iter_predicate!()
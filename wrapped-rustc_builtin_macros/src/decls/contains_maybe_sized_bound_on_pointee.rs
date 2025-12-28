macro_rules! contains_maybe_sized_bound_on_pointee {
    () => {
        fn contains_maybe_sized_bound_on_pointee (predicates : & [WherePredicate] , pointee : Symbol) -> bool { for bound in predicates { if let ast :: WherePredicateKind :: BoundPredicate (bound) = & bound . kind && bound . bounded_ty . kind . is_simple_path () . is_some_and (| name | name == pointee) { for bound in & bound . bounds { if is_maybe_sized_bound (bound) { return true ; } } } } false }
    };
}

contains_maybe_sized_bound_on_pointee!()
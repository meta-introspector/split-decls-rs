macro_rules! impl_66 {
    () => {
        impl TypeAliasBounds { pub (crate) fn affects_object_lifetime_defaults (pred : & hir :: WherePredicate < '_ >) -> bool { if let hir :: WherePredicateKind :: BoundPredicate (pred) = pred . kind && pred . bounds . iter () . any (| bound | matches ! (bound , hir :: GenericBound :: Outlives (_))) && pred . bound_generic_params . is_empty () && pred . bounded_ty . as_generic_param () . is_some () { return true ; } false } }
    };
}

impl_66!()
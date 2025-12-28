macro_rules! search_bounds_for {
    () => {
        fn search_bounds_for < 'tcx > (hir_bounds : & 'tcx [hir :: GenericBound < 'tcx >] , self_ty_where_predicates : Option < (LocalDefId , & 'tcx [hir :: WherePredicate < 'tcx >]) > , mut f : impl FnMut (& 'tcx PolyTraitRef < 'tcx >) ,) { let mut search_bounds = | hir_bounds : & 'tcx [hir :: GenericBound < 'tcx >] | { for hir_bound in hir_bounds { let hir :: GenericBound :: Trait (ptr) = hir_bound else { continue ; } ; f (ptr) } } ; search_bounds (hir_bounds) ; if let Some ((self_ty , where_clause)) = self_ty_where_predicates { for clause in where_clause { if let hir :: WherePredicateKind :: BoundPredicate (pred) = clause . kind && pred . is_param_bound (self_ty . to_def_id ()) { search_bounds (pred . bounds) ; } } } }
    };
}

search_bounds_for!()
macro_rules! collect_relaxed_bounds {
    () => {
        fn collect_relaxed_bounds < 'tcx > (hir_bounds : & 'tcx [hir :: GenericBound < 'tcx >] , self_ty_where_predicates : Option < (LocalDefId , & 'tcx [hir :: WherePredicate < 'tcx >]) > ,) -> SmallVec < [& 'tcx PolyTraitRef < 'tcx > ; 1] > { let mut relaxed_bounds : SmallVec < [_ ; 1] > = SmallVec :: new () ; search_bounds_for (hir_bounds , self_ty_where_predicates , | ptr | { if matches ! (ptr . modifiers . polarity , hir :: BoundPolarity :: Maybe (_)) { relaxed_bounds . push (ptr) ; } }) ; relaxed_bounds }
    };
}

collect_relaxed_bounds!()
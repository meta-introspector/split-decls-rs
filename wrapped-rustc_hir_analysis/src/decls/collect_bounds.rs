macro_rules! deps {
    () => {
        CollectedBound!();
    };
}

macro_rules! collect_bounds {
    () => {
        deps!();
        fn collect_bounds < 'a , 'tcx > (hir_bounds : & 'a [hir :: GenericBound < 'tcx >] , self_ty_where_predicates : Option < (LocalDefId , & 'tcx [hir :: WherePredicate < 'tcx >]) > , target_did : DefId ,) -> CollectedBound { let mut collect_into = CollectedBound :: default () ; search_bounds_for (hir_bounds , self_ty_where_predicates , | ptr | { if ! matches ! (ptr . trait_ref . path . res , Res :: Def (DefKind :: Trait , did) if did == target_did) { return ; } match ptr . modifiers . polarity { hir :: BoundPolarity :: Maybe (_) => collect_into . maybe = true , hir :: BoundPolarity :: Negative (_) => collect_into . negative = true , hir :: BoundPolarity :: Positive => collect_into . positive = true , } }) ; collect_into }
    };
}

collect_bounds!();
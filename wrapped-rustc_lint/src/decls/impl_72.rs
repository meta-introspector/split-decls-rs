macro_rules! deps {
    () => {
        LateContext!();
        BuiltinTrivialBounds!();
        UnstableFeature!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for TrivialConstraints { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: Item < 'tcx >) { use rustc_middle :: ty :: ClauseKind ; if cx . tcx . features () . trivial_bounds () { let predicates = cx . tcx . predicates_of (item . owner_id) ; for & (predicate , span) in predicates . predicates { let predicate_kind_name = match predicate . kind () . skip_binder () { ClauseKind :: Trait (..) => "trait" , ClauseKind :: TypeOutlives (..) | ClauseKind :: RegionOutlives (..) => "lifetime" , ClauseKind :: UnstableFeature (_) | ClauseKind :: ConstArgHasType (..) | ClauseKind :: Projection (..) | ClauseKind :: WellFormed (..) | ClauseKind :: ConstEvaluatable (..) | ty :: ClauseKind :: HostEffect (..) => continue , } ; if predicate . is_global () { cx . emit_span_lint (TRIVIAL_BOUNDS , span , BuiltinTrivialBounds { predicate_kind_name , predicate } ,) ; } } } } }
    };
}

impl_72!()
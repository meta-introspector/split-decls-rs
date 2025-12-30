// Generated macro for inferred_outlives_crate (function)
macro_rules! Depcrate_outlivesinferred_outlives_crate {
() => {
// Module: crate::outlives
// Provides: {"inferred_outlives_crate"}
// Dependencies: {}
pub (super) fn inferred_outlives_crate (tcx : TyCtxt < '_ > , () : ()) -> CratePredicatesMap < '_ > { let global_inferred_outlives = implicit_infer :: infer_predicates (tcx) ; let predicates = global_inferred_outlives . iter () . map (| (& def_id , set) | { let predicates = & * tcx . arena . alloc_from_iter (set . as_ref () . skip_binder () . iter () . filter_map (| (ty :: OutlivesPredicate (arg1 , region2) , & span) | { match arg1 . kind () { GenericArgKind :: Type (ty1) => Some ((ty :: ClauseKind :: TypeOutlives (ty :: OutlivesPredicate (ty1 , * region2)) . upcast (tcx) , span ,)) , GenericArgKind :: Lifetime (region1) => Some ((ty :: ClauseKind :: RegionOutlives (ty :: OutlivesPredicate (region1 , * region2 ,)) . upcast (tcx) , span ,)) , GenericArgKind :: Const (_) => { None } } } ,)) ; (def_id , predicates) }) . collect () ; ty :: CratePredicatesMap { predicates } }
};
}

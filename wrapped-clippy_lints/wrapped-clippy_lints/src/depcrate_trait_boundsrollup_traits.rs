// Generated macro for rollup_traits (function)
macro_rules! Depcrate_trait_boundsrollup_traits {
() => {
// Module: crate::trait_bounds
// Provides: {"rollup_traits"}
// Dependencies: {}
fn rollup_traits < 'cx , 'tcx > (cx : & 'cx LateContext < 'tcx > , bounds : & 'tcx [GenericBound < 'tcx >] , msg : & 'static str ,) -> Vec < (ComparableTraitRef < 'cx , 'tcx > , Span) > { let mut map = FxIndexMap :: default () ; let mut repeated_res = false ; let only_comparable_trait_refs = | bound : & 'tcx GenericBound < 'tcx > | { if let GenericBound :: Trait (t) = bound { Some ((ComparableTraitRef { cx , trait_ref : & t . trait_ref , modifiers : t . modifiers , } , t . span ,)) } else { None } } ; for bound in bounds . iter () . filter_map (only_comparable_trait_refs) { let (comparable_bound , span_direct) = bound ; match map . entry (comparable_bound) { IndexEntry :: Occupied (_) => repeated_res = true , IndexEntry :: Vacant (e) => { e . insert (span_direct) ; } , } } let comparable_bounds : Vec < _ > = map . into_iter () . collect () ; if repeated_res && let [first_trait , .. , last_trait] = bounds { let all_trait_span = first_trait . span () . to (last_trait . span ()) ; let traits = comparable_bounds . iter () . filter_map (| & (_ , span) | span . get_source_text (cx)) . join (" + ") ; span_lint_and_sugg (cx , TRAIT_DUPLICATION_IN_BOUNDS , all_trait_span , msg , "try" , traits , Applicability :: MachineApplicable ,) ; } comparable_bounds }
};
}

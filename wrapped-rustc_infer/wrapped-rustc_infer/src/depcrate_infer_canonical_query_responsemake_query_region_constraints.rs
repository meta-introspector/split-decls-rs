// Generated macro for make_query_region_constraints (function)
macro_rules! Depcrate_infer_canonical_query_responsemake_query_region_constraints {
() => {
// Module: crate::infer::canonical::query_response
// Provides: {"make_query_region_constraints"}
// Dependencies: {}
# [doc = " Given the region obligations and constraints scraped from the infcx,"] # [doc = " creates query region constraints."] pub fn make_query_region_constraints < 'tcx > (outlives_obligations : Vec < TypeOutlivesConstraint < 'tcx > > , region_constraints : & RegionConstraintData < 'tcx > , assumptions : Vec < ty :: ArgOutlivesPredicate < 'tcx > > ,) -> QueryRegionConstraints < 'tcx > { let RegionConstraintData { constraints , verifys } = region_constraints ; assert ! (verifys . is_empty ()) ; debug ! (? constraints) ; let outlives : Vec < _ > = constraints . iter () . map (| (c , origin) | { let constraint = ty :: OutlivesPredicate (c . sup . into () , c . sub) ; (constraint , origin . to_constraint_category ()) }) . chain (outlives_obligations . into_iter () . map (| obl | { (ty :: OutlivesPredicate (obl . sup_type . into () , obl . sub_region) , obl . origin . to_constraint_category () ,) })) . collect () ; QueryRegionConstraints { outlives , assumptions } }
};
}

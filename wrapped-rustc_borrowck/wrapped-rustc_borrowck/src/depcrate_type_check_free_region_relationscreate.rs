// Generated macro for create (function)
macro_rules! Depcrate_type_check_free_region_relationscreate {
() => {
// Module: crate::type_check::free_region_relations
// Provides: {"create"}
// Dependencies: {}
pub (crate) fn create < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , universal_regions : UniversalRegions < 'tcx > , constraints : & mut MirTypeckRegionConstraints < 'tcx > ,) -> CreateResult < 'tcx > { UniversalRegionRelationsBuilder { infcx , constraints , universal_regions , region_bound_pairs : Default :: default () , outlives : Default :: default () , inverse_outlives : Default :: default () , } . create () }
};
}

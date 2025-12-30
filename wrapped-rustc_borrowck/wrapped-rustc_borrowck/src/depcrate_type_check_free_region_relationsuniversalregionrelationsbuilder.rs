// Generated macro for UniversalRegionRelationsBuilder (struct)
macro_rules! Depcrate_type_check_free_region_relationsUniversalRegionRelationsBuilder {
() => {
// Module: crate::type_check::free_region_relations
// Provides: {"UniversalRegionRelationsBuilder"}
// Dependencies: {}
struct UniversalRegionRelationsBuilder < 'a , 'tcx > { infcx : & 'a BorrowckInferCtxt < 'tcx > , universal_regions : UniversalRegions < 'tcx > , constraints : & 'a mut MirTypeckRegionConstraints < 'tcx > , outlives : TransitiveRelationBuilder < RegionVid > , inverse_outlives : TransitiveRelationBuilder < RegionVid > , region_bound_pairs : RegionBoundPairs < 'tcx > , }
};
}

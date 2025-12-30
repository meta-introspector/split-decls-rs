// Generated macro for UniversalRegionRelations (struct)
macro_rules! Depcrate_type_check_free_region_relationsUniversalRegionRelations {
() => {
// Module: crate::type_check::free_region_relations
// Provides: {"UniversalRegionRelations"}
// Dependencies: {}
# [derive (Debug)] # [derive (Clone)] pub (crate) struct UniversalRegionRelations < 'tcx > { pub (crate) universal_regions : UniversalRegions < 'tcx > , # [doc = " Stores the outlives relations that are known to hold from the"] # [doc = " implied bounds, in-scope where-clauses, and that sort of"] # [doc = " thing."] outlives : TransitiveRelation < RegionVid > , # [doc = " This is the `<=` relation; that is, if `a: b`, then `b <= a`,"] # [doc = " and we store that here. This is useful when figuring out how"] # [doc = " to express some local region in terms of external regions our"] # [doc = " caller will understand."] inverse_outlives : TransitiveRelation < RegionVid > , }
};
}

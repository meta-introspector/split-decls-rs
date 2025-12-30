// Generated macro for CreateResult (struct)
macro_rules! Depcrate_type_check_free_region_relationsCreateResult {
() => {
// Module: crate::type_check::free_region_relations
// Provides: {"CreateResult"}
// Dependencies: {}
pub (crate) struct CreateResult < 'tcx > { pub (crate) universal_region_relations : Frozen < UniversalRegionRelations < 'tcx > > , pub (crate) region_bound_pairs : Frozen < RegionBoundPairs < 'tcx > > , pub (crate) known_type_outlives_obligations : Frozen < Vec < ty :: PolyTypeOutlivesPredicate < 'tcx > > > , pub (crate) normalized_inputs_and_output : NormalizedInputsAndOutput < 'tcx > , }
};
}

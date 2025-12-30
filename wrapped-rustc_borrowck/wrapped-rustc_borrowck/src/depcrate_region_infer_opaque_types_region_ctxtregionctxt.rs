// Generated macro for RegionCtxt (struct)
macro_rules! Depcrate_region_infer_opaque_types_region_ctxtRegionCtxt {
() => {
// Module: crate::region_infer::opaque_types::region_ctxt
// Provides: {"RegionCtxt"}
// Dependencies: {}
# [doc = " A slimmed down version of [crate::region_infer::RegionInferenceContext] used"] # [doc = " only by opaque type handling."] pub (super) struct RegionCtxt < 'a , 'tcx > { pub (super) infcx : & 'a BorrowckInferCtxt < 'tcx > , pub (super) definitions : Frozen < IndexVec < RegionVid , RegionDefinition < 'tcx > > > , pub (super) universal_region_relations : & 'a UniversalRegionRelations < 'tcx > , pub (super) constraint_sccs : ConstraintSccs , pub (super) scc_annotations : IndexVec < ConstraintSccIndex , RegionTracker > , pub (super) rev_scc_graph : ReverseSccGraph , pub (super) scc_values : RegionValues < ConstraintSccIndex > , }
};
}

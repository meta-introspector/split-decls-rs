// Generated macro for SccConstraints (struct)
macro_rules! Depcrate_region_infer_graphvizSccConstraints {
() => {
// Module: crate::region_infer::graphviz
// Provides: {"SccConstraints"}
// Dependencies: {}
struct SccConstraints < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , regioncx : & 'a RegionInferenceContext < 'tcx > , nodes_per_scc : IndexVec < ConstraintSccIndex , Vec < RegionVid > > , }
};
}

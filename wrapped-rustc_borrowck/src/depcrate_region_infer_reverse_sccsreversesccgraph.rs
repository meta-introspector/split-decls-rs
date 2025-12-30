// Generated macro for ReverseSccGraph (struct)
macro_rules! Depcrate_region_infer_reverse_sccsReverseSccGraph {
() => {
// Module: crate::region_infer::reverse_sccs
// Provides: {"ReverseSccGraph"}
// Dependencies: {}
pub (crate) struct ReverseSccGraph { graph : VecGraph < ConstraintSccIndex > , # [doc = " For each SCC, the range of `universal_regions` that use that SCC as"] # [doc = " their value."] scc_regions : FxIndexMap < ConstraintSccIndex , Range < usize > > , # [doc = " All of the universal regions, in grouped so that `scc_regions` can"] # [doc = " index into here."] universal_regions : Vec < RegionVid > , }
};
}

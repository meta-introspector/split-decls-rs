// Generated macro for impl_860 (impl)
macro_rules! Depcrate_region_infer_graphvizimpl_860 {
() => {
// Module: crate::region_infer::graphviz
// Provides: {"impl_860"}
// Dependencies: {}
impl < 'tcx > RegionInferenceContext < 'tcx > { # [doc = " Write out the region constraint graph."] pub (crate) fn dump_graphviz_raw_constraints (& self , tcx : TyCtxt < 'tcx > , mut w : & mut dyn Write ,) -> io :: Result < () > { dot :: render (& RawConstraints { tcx , regioncx : self } , & mut w) } # [doc = " Write out the region constraint SCC graph."] pub (crate) fn dump_graphviz_scc_constraints (& self , tcx : TyCtxt < 'tcx > , mut w : & mut dyn Write ,) -> io :: Result < () > { let mut nodes_per_scc : IndexVec < ConstraintSccIndex , _ > = self . constraint_sccs . all_sccs () . map (| _ | Vec :: new ()) . collect () ; for region in self . definitions . indices () { let scc = self . constraint_sccs . scc (region) ; nodes_per_scc [scc] . push (region) ; } dot :: render (& SccConstraints { tcx , regioncx : self , nodes_per_scc } , & mut w) } }
};
}

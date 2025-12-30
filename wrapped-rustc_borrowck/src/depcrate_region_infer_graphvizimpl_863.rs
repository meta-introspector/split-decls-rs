// Generated macro for impl_863 (impl)
macro_rules! Depcrate_region_infer_graphvizimpl_863 {
() => {
// Module: crate::region_infer::graphviz
// Provides: {"impl_863"}
// Dependencies: {}
impl < 'a , 'this , 'tcx > dot :: GraphWalk < 'this > for RawConstraints < 'a , 'tcx > { type Node = RegionVid ; type Edge = OutlivesConstraint < 'tcx > ; fn nodes (& 'this self) -> dot :: Nodes < 'this , RegionVid > { let vids : Vec < RegionVid > = self . regioncx . definitions . indices () . collect () ; vids . into () } fn edges (& 'this self) -> dot :: Edges < 'this , OutlivesConstraint < 'tcx > > { (& self . regioncx . constraints . outlives () . raw [..]) . into () } fn source (& 'this self , edge : & OutlivesConstraint < 'tcx >) -> RegionVid { edge . sup } fn target (& 'this self , edge : & OutlivesConstraint < 'tcx >) -> RegionVid { edge . sub } }
};
}

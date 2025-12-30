// Generated macro for impl_114 (impl)
macro_rules! Depcrate_constraints_graphimpl_114 {
() => {
// Module: crate::constraints::graph
// Provides: {"impl_114"}
// Dependencies: {}
impl < 'a , 'tcx , D : ConstraintGraphDirection > Iterator for Successors < 'a , 'tcx , D > { type Item = RegionVid ; fn next (& mut self) -> Option < Self :: Item > { match self { Successors :: FromStatic (edges) => { edges . next () } Successors :: FromGraph (edges) => { edges . next () . map (| constraint | D :: end_region (constraint . sup , constraint . sub)) } } } }
};
}

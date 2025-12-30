// Generated macro for impl_108 (impl)
macro_rules! Depcrate_constraints_graphimpl_108 {
() => {
// Module: crate::constraints::graph
// Provides: {"impl_108"}
// Dependencies: {}
impl < 'a , 'tcx , D : ConstraintGraphDirection > Iterator for EdgesFromGraph < 'a , 'tcx , D > { type Item = & 'a OutlivesConstraint < 'tcx > ; fn next (& mut self) -> Option < Self :: Item > { if let Some (p) = self . pointer { self . pointer = self . graph . next_constraints [p] ; Some (& self . constraints [p]) } else { None } } }
};
}

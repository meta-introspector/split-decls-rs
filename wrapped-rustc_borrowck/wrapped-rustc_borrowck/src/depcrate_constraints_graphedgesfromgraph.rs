// Generated macro for EdgesFromGraph (struct)
macro_rules! Depcrate_constraints_graphEdgesFromGraph {
() => {
// Module: crate::constraints::graph
// Provides: {"EdgesFromGraph"}
// Dependencies: {}
pub (crate) struct EdgesFromGraph < 'a , 'tcx , D : ConstraintGraphDirection > { graph : & 'a ConstraintGraph < D > , constraints : & 'a OutlivesConstraintSet < 'tcx > , pointer : Option < OutlivesConstraintIndex > , }
};
}

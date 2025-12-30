// Generated macro for Successors (enum)
macro_rules! Depcrate_constraints_graphSuccessors {
() => {
// Module: crate::constraints::graph
// Provides: {"Successors"}
// Dependencies: {}
pub (crate) enum Successors < 'a , 'tcx , D : ConstraintGraphDirection > { FromStatic (EdgesFromStatic) , FromGraph (EdgesFromGraph < 'a , 'tcx , D >) , }
};
}

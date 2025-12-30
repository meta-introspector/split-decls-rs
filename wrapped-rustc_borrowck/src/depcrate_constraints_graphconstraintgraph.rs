// Generated macro for ConstraintGraph (struct)
macro_rules! Depcrate_constraints_graphConstraintGraph {
() => {
// Module: crate::constraints::graph
// Provides: {"ConstraintGraph"}
// Dependencies: {}
# [doc = " The construct graph organizes the constraints by their end-points."] # [doc = " It can be used to view a `R1: R2` constraint as either an edge `R1"] # [doc = " -> R2` or `R2 -> R1` depending on the direction type `D`."] pub (crate) struct ConstraintGraph < D : ConstraintGraphDirection > { _direction : D , first_constraints : IndexVec < RegionVid , Option < OutlivesConstraintIndex > > , next_constraints : IndexVec < OutlivesConstraintIndex , Option < OutlivesConstraintIndex > > , }
};
}

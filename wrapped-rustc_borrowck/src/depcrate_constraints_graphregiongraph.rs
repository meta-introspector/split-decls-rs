// Generated macro for RegionGraph (struct)
macro_rules! Depcrate_constraints_graphRegionGraph {
() => {
// Module: crate::constraints::graph
// Provides: {"RegionGraph"}
// Dependencies: {}
# [doc = " This struct brings together a constraint set and a (normal, not"] # [doc = " reverse) constraint graph. It implements the graph traits and is"] # [doc = " usd for doing the SCC computation."] pub (crate) struct RegionGraph < 'a , 'tcx , D : ConstraintGraphDirection > { set : & 'a OutlivesConstraintSet < 'tcx > , constraint_graph : & 'a ConstraintGraph < D > , static_region : RegionVid , }
};
}

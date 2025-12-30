// Generated macro for ConstraintGraphDirection (trait)
macro_rules! Depcrate_constraints_graphConstraintGraphDirection {
() => {
// Module: crate::constraints::graph
// Provides: {"ConstraintGraphDirection"}
// Dependencies: {}
# [doc = " Marker trait that controls whether a `R1: R2` constraint"] # [doc = " represents an edge `R1 -> R2` or `R2 -> R1`."] pub (crate) trait ConstraintGraphDirection : Copy + 'static { fn start_region (sup : RegionVid , sub : RegionVid) -> RegionVid ; fn end_region (sup : RegionVid , sub : RegionVid) -> RegionVid ; fn is_normal () -> bool ; }
};
}

// Generated macro for EdgeFilteredNeighbors (struct)
macro_rules! Depcrate_visit_filterEdgeFilteredNeighbors {
() => {
// Module: crate::visit::filter
// Provides: {"EdgeFilteredNeighbors"}
// Dependencies: {}
# [doc = " A filtered neighbors iterator."] # [derive (Debug , Clone)] pub struct EdgeFilteredNeighbors < 'a , G , F : 'a > where G : IntoEdges , { iter : G :: Edges , f : & 'a F , }
};
}

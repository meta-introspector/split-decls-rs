// Generated macro for EdgeFilteredNeighborsDirected (struct)
macro_rules! Depcrate_visit_filterEdgeFilteredNeighborsDirected {
() => {
// Module: crate::visit::filter
// Provides: {"EdgeFilteredNeighborsDirected"}
// Dependencies: {}
# [doc = " A filtered neighbors-directed iterator."] # [derive (Debug , Clone)] pub struct EdgeFilteredNeighborsDirected < 'a , G , F : 'a > where G : IntoEdgesDirected , { iter : G :: EdgesDirected , f : & 'a F , from : G :: NodeId , }
};
}

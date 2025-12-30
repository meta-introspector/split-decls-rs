// Generated macro for EdgeReference (struct)
macro_rules! Depcrate_graph_impl_stable_graphEdgeReference {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"EdgeReference"}
// Dependencies: {}
# [doc = " Reference to a `StableGraph` edge."] # [derive (Debug)] pub struct EdgeReference < 'a , E : 'a , Ix = DefaultIx > { index : EdgeIndex < Ix > , node : [NodeIndex < Ix > ; 2] , weight : & 'a E , }
};
}

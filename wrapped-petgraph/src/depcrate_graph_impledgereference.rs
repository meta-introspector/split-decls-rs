// Generated macro for EdgeReference (struct)
macro_rules! Depcrate_graph_implEdgeReference {
() => {
// Module: crate::graph_impl
// Provides: {"EdgeReference"}
// Dependencies: {}
# [doc = " Reference to a `Graph` edge."] # [derive (Debug)] pub struct EdgeReference < 'a , E : 'a , Ix = DefaultIx > { index : EdgeIndex < Ix > , node : [NodeIndex < Ix > ; 2] , weight : & 'a E , }
};
}

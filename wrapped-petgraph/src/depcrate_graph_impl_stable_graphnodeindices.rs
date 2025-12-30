// Generated macro for NodeIndices (struct)
macro_rules! Depcrate_graph_impl_stable_graphNodeIndices {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"NodeIndices"}
// Dependencies: {}
# [doc = " Iterator over the node indices of a graph."] # [derive (Debug , Clone)] pub struct NodeIndices < 'a , N : 'a , Ix : 'a = DefaultIx > { iter : iter :: Enumerate < slice :: Iter < 'a , Node < Option < N > , Ix > > > , }
};
}

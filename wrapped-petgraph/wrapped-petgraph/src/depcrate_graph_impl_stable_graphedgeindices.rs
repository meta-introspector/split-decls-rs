// Generated macro for EdgeIndices (struct)
macro_rules! Depcrate_graph_impl_stable_graphEdgeIndices {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"EdgeIndices"}
// Dependencies: {}
# [doc = " Iterator over the edge indices of a graph."] # [doc = ""] # [doc = " Note: `EdgeIndices` borrows a graph."] # [derive (Debug , Clone)] pub struct EdgeIndices < 'a , E : 'a , Ix : 'a = DefaultIx > { iter : iter :: Enumerate < slice :: Iter < 'a , Edge < Option < E > , Ix > > > , }
};
}

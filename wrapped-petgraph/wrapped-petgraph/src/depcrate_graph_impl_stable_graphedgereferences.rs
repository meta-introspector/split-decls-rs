// Generated macro for EdgeReferences (struct)
macro_rules! Depcrate_graph_impl_stable_graphEdgeReferences {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"EdgeReferences"}
// Dependencies: {}
# [doc = " Iterator over all edges of a graph."] # [derive (Debug , Clone)] pub struct EdgeReferences < 'a , E : 'a , Ix : 'a = DefaultIx > { iter : iter :: Enumerate < slice :: Iter < 'a , Edge < Option < E > , Ix > > > , }
};
}

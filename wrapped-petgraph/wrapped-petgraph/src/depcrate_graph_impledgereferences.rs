// Generated macro for EdgeReferences (struct)
macro_rules! Depcrate_graph_implEdgeReferences {
() => {
// Module: crate::graph_impl
// Provides: {"EdgeReferences"}
// Dependencies: {}
# [doc = " Iterator over all edges of a graph."] # [derive (Debug , Clone)] pub struct EdgeReferences < 'a , E : 'a , Ix : IndexType = DefaultIx > { iter : iter :: Enumerate < slice :: Iter < 'a , Edge < E , Ix > > > , }
};
}

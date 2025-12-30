// Generated macro for NodeReferences (struct)
macro_rules! Depcrate_graph_implNodeReferences {
() => {
// Module: crate::graph_impl
// Provides: {"NodeReferences"}
// Dependencies: {}
# [doc = " Iterator over all nodes of a graph."] # [derive (Debug , Clone)] pub struct NodeReferences < 'a , N : 'a , Ix : IndexType = DefaultIx > { iter : iter :: Enumerate < slice :: Iter < 'a , Node < N , Ix > > > , }
};
}

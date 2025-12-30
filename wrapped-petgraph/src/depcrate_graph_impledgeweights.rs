// Generated macro for EdgeWeights (struct)
macro_rules! Depcrate_graph_implEdgeWeights {
() => {
// Module: crate::graph_impl
// Provides: {"EdgeWeights"}
// Dependencies: {}
# [doc = " Iterator yielding immutable access to all edge weights."] pub struct EdgeWeights < 'a , E : 'a , Ix : IndexType = DefaultIx > { edges : :: core :: slice :: Iter < 'a , Edge < E , Ix > > , }
};
}

// Generated macro for EdgeWeightsMut (struct)
macro_rules! Depcrate_graph_implEdgeWeightsMut {
() => {
// Module: crate::graph_impl
// Provides: {"EdgeWeightsMut"}
// Dependencies: {}
# [doc = " Iterator yielding mutable access to all edge weights."] # [derive (Debug)] pub struct EdgeWeightsMut < 'a , E : 'a , Ix : IndexType = DefaultIx > { edges : :: core :: slice :: IterMut < 'a , Edge < E , Ix > > , }
};
}

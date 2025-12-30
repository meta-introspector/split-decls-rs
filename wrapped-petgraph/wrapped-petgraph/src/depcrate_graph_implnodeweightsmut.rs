// Generated macro for NodeWeightsMut (struct)
macro_rules! Depcrate_graph_implNodeWeightsMut {
() => {
// Module: crate::graph_impl
// Provides: {"NodeWeightsMut"}
// Dependencies: {}
# [doc = " Iterator yielding mutable access to all node weights."] # [derive (Debug)] pub struct NodeWeightsMut < 'a , N : 'a , Ix : IndexType = DefaultIx > { nodes : :: core :: slice :: IterMut < 'a , Node < N , Ix > > , }
};
}

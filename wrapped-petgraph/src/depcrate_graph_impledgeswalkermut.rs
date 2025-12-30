// Generated macro for EdgesWalkerMut (struct)
macro_rules! Depcrate_graph_implEdgesWalkerMut {
() => {
// Module: crate::graph_impl
// Provides: {"EdgesWalkerMut"}
// Dependencies: {}
struct EdgesWalkerMut < 'a , E : 'a , Ix : IndexType = DefaultIx > { edges : & 'a mut [Edge < E , Ix >] , next : EdgeIndex < Ix > , dir : Direction , }
};
}

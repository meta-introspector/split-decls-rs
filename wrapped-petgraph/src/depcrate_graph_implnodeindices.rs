// Generated macro for NodeIndices (struct)
macro_rules! Depcrate_graph_implNodeIndices {
() => {
// Module: crate::graph_impl
// Provides: {"NodeIndices"}
// Dependencies: {}
# [doc = " Iterator over the node indices of a graph."] # [derive (Clone , Debug)] pub struct NodeIndices < Ix = DefaultIx > { r : Range < usize > , ty : PhantomData < fn () -> Ix > , }
};
}

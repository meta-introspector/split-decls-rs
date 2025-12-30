// Generated macro for EdgeIndices (struct)
macro_rules! Depcrate_graph_implEdgeIndices {
() => {
// Module: crate::graph_impl
// Provides: {"EdgeIndices"}
// Dependencies: {}
# [doc = " Iterator over the edge indices of a graph."] # [derive (Clone , Debug)] pub struct EdgeIndices < Ix = DefaultIx > { r : Range < usize > , ty : PhantomData < fn () -> Ix > , }
};
}

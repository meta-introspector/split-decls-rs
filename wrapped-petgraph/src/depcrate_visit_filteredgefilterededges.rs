// Generated macro for EdgeFilteredEdges (struct)
macro_rules! Depcrate_visit_filterEdgeFilteredEdges {
() => {
// Module: crate::visit::filter
// Provides: {"EdgeFilteredEdges"}
// Dependencies: {}
# [doc = " A filtered edges iterator."] # [derive (Debug , Clone)] pub struct EdgeFilteredEdges < 'a , G , I , F : 'a > { graph : PhantomData < G > , iter : I , f : & 'a F , }
};
}

// Generated macro for NodeFilteredEdges (struct)
macro_rules! Depcrate_visit_filterNodeFilteredEdges {
() => {
// Module: crate::visit::filter
// Provides: {"NodeFilteredEdges"}
// Dependencies: {}
# [doc = " A filtered edges iterator."] # [derive (Debug , Clone)] pub struct NodeFilteredEdges < 'a , G , I , F : 'a > { graph : PhantomData < G > , include_source : bool , iter : I , f : & 'a F , dir : Direction , }
};
}

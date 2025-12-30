// Generated macro for NodeFilteredEdgeReferences (struct)
macro_rules! Depcrate_visit_filterNodeFilteredEdgeReferences {
() => {
// Module: crate::visit::filter
// Provides: {"NodeFilteredEdgeReferences"}
// Dependencies: {}
# [doc = " A filtered edges iterator."] # [derive (Debug , Clone)] pub struct NodeFilteredEdgeReferences < 'a , G , I , F : 'a > { graph : PhantomData < G > , iter : I , f : & 'a F , }
};
}

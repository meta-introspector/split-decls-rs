// Generated macro for impl_364 (impl)
macro_rules! Depcrate_adjimpl_364 {
() => {
// Module: crate::adj
// Provides: {"impl_364"}
// Dependencies: {}
impl < 'a , Ix : IndexType , E > visit :: IntoEdges for & 'a List < E , Ix > { type Edges = OutgoingEdgeReferences < 'a , E , Ix > ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { let iter = self . suc [a . index ()] . iter () . enumerate () . zip (core :: iter :: repeat (a)) . map (proj1 as _) ; OutgoingEdgeReferences { iter } } }
};
}

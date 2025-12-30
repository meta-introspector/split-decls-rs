// Generated macro for impl_362 (impl)
macro_rules! Depcrate_adjimpl_362 {
() => {
// Module: crate::adj
// Provides: {"impl_362"}
// Dependencies: {}
impl < 'a , Ix : IndexType , E > visit :: IntoEdgeReferences for & 'a List < E , Ix > { type EdgeRef = EdgeReference < 'a , E , Ix > ; type EdgeReferences = EdgeReferences < 'a , E , Ix > ; fn edge_references (self) -> Self :: EdgeReferences { let iter = self . suc . iter () . enumerate () . flat_map (proj2 as _) ; EdgeReferences { iter } } }
};
}

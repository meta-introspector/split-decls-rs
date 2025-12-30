// Generated macro for impl_1423 (impl)
macro_rules! Depcrateimpl_1423 {
() => {
// Module: crate
// Provides: {"impl_1423"}
// Dependencies: {}
impl < Ix , E > IntoWeightedEdge < E > for (Ix , Ix , & E) where E : Clone , { type NodeId = Ix ; fn into_weighted_edge (self) -> (Ix , Ix , E) { let (a , b , c) = self ; (a , b , c . clone ()) } }
};
}

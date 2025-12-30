// Generated macro for impl_1425 (impl)
macro_rules! Depcrateimpl_1425 {
() => {
// Module: crate
// Provides: {"impl_1425"}
// Dependencies: {}
impl < Ix , E > IntoWeightedEdge < E > for & (Ix , Ix , E) where Ix : Copy , E : Clone , { type NodeId = Ix ; fn into_weighted_edge (self) -> (Ix , Ix , E) { self . clone () } }
};
}

// Generated macro for impl_1421 (impl)
macro_rules! Depcrateimpl_1421 {
() => {
// Module: crate
// Provides: {"impl_1421"}
// Dependencies: {}
impl < Ix , E > IntoWeightedEdge < E > for (Ix , Ix) where E : Default , { type NodeId = Ix ; fn into_weighted_edge (self) -> (Ix , Ix , E) { let (s , t) = self ; (s , t , E :: default ()) } }
};
}

// Generated macro for impl_1424 (impl)
macro_rules! Depcrateimpl_1424 {
() => {
// Module: crate
// Provides: {"impl_1424"}
// Dependencies: {}
impl < Ix , E > IntoWeightedEdge < E > for & (Ix , Ix) where Ix : Copy , E : Default , { type NodeId = Ix ; fn into_weighted_edge (self) -> (Ix , Ix , E) { let (s , t) = * self ; (s , t , E :: default ()) } }
};
}

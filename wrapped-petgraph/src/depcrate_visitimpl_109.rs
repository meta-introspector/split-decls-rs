// Generated macro for impl_109 (impl)
macro_rules! Depcrate_visitimpl_109 {
() => {
// Module: crate::visit
// Provides: {"impl_109"}
// Dependencies: {}
impl < Id , W > NodeRef for (Id , & W) where Id : Copy , { type NodeId = Id ; type Weight = W ; fn id (& self) -> Self :: NodeId { self . 0 } fn weight (& self) -> & Self :: Weight { self . 1 } }
};
}

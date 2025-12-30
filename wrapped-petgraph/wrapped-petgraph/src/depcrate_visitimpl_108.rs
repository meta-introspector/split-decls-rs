// Generated macro for impl_108 (impl)
macro_rules! Depcrate_visitimpl_108 {
() => {
// Module: crate::visit
// Provides: {"impl_108"}
// Dependencies: {}
impl < Id > NodeRef for (Id , ()) where Id : Copy , { type NodeId = Id ; type Weight = () ; fn id (& self) -> Self :: NodeId { self . 0 } fn weight (& self) -> & Self :: Weight { static DUMMY : () = () ; & DUMMY } }
};
}

// Generated macro for impl_104 (impl)
macro_rules! Depcrate_visitimpl_104 {
() => {
// Module: crate::visit
// Provides: {"impl_104"}
// Dependencies: {}
impl < N , E > EdgeRef for (N , N , & E) where N : Copy , { type NodeId = N ; type EdgeId = (N , N) ; type Weight = E ; fn source (& self) -> N { self . 0 } fn target (& self) -> N { self . 1 } fn weight (& self) -> & E { self . 2 } fn id (& self) -> (N , N) { (self . 0 , self . 1) } }
};
}

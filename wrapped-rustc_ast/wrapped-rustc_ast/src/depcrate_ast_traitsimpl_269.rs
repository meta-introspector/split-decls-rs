// Generated macro for impl_269 (impl)
macro_rules! Depcrate_ast_traitsimpl_269 {
() => {
// Module: crate::ast_traits
// Provides: {"impl_269"}
// Dependencies: {}
impl < T : HasNodeId > HasNodeId for Box < T > { fn node_id (& self) -> NodeId { (* * self) . node_id () } fn node_id_mut (& mut self) -> & mut NodeId { (* * self) . node_id_mut () } }
};
}

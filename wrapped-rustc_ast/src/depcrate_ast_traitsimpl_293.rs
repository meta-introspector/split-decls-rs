// Generated macro for impl_293 (impl)
macro_rules! Depcrate_ast_traitsimpl_293 {
() => {
// Module: crate::ast_traits
// Provides: {"impl_293"}
// Dependencies: {}
impl < Wrapped : HasNodeId , Tag > HasNodeId for AstNodeWrapper < Wrapped , Tag > { fn node_id (& self) -> NodeId { self . wrapped . node_id () } fn node_id_mut (& mut self) -> & mut NodeId { self . wrapped . node_id_mut () } }
};
}

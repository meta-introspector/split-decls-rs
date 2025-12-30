// Generated macro for impl_has_node_id (macro)
macro_rules! Depcrate_ast_traitsimpl_has_node_id {
() => {
// Module: crate::ast_traits
// Provides: {"impl_has_node_id"}
// Dependencies: {}
macro_rules ! impl_has_node_id { ($ ($ T : ty) ,+ $ (,) ?) => { $ (impl HasNodeId for $ T { fn node_id (& self) -> NodeId { self . id } fn node_id_mut (& mut self) -> & mut NodeId { & mut self . id } }) + } ; }
};
}

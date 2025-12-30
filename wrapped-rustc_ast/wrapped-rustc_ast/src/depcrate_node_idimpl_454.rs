// Generated macro for impl_454 (impl)
macro_rules! Depcrate_node_idimpl_454 {
() => {
// Module: crate::node_id
// Provides: {"impl_454"}
// Dependencies: {}
impl NodeId { pub fn placeholder_from_expn_id (expn_id : LocalExpnId) -> Self { NodeId :: from_u32 (expn_id . as_u32 ()) } pub fn placeholder_to_expn_id (self) -> LocalExpnId { LocalExpnId :: from_u32 (self . as_u32 ()) } }
};
}

// Generated macro for impl_623 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_623 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_623"}
// Dependencies: {}
impl < Node , Type > Handle < Node , Type > { # [doc = " Retrieves the node that contains the edge or key-value pair this handle points to."] pub (super) fn into_node (self) -> Node { self . node } # [doc = " Returns the position of this handle in the node."] pub (super) fn idx (& self) -> usize { self . idx } }
};
}

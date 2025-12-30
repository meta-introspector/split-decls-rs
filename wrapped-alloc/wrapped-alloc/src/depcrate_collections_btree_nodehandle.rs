// Generated macro for Handle (struct)
macro_rules! Depcrate_collections_btree_nodeHandle {
() => {
// Module: crate::collections::btree::node
// Provides: {"Handle"}
// Dependencies: {}
# [doc = " A reference to a specific key-value pair or edge within a node. The `Node` parameter"] # [doc = " must be a `NodeRef`, while the `Type` can either be `KV` (signifying a handle on a key-value"] # [doc = " pair) or `Edge` (signifying a handle on an edge)."] # [doc = ""] # [doc = " Note that even `Leaf` nodes can have `Edge` handles. Instead of representing a pointer to"] # [doc = " a child node, these represent the spaces where child pointers would go between the key-value"] # [doc = " pairs. For example, in a node with length 2, there would be 3 possible edge locations - one"] # [doc = " to the left of the node, one between the two pairs, and one at the right of the node."] pub (super) struct Handle < Node , Type > { node : Node , idx : usize , _marker : PhantomData < Type > , }
};
}

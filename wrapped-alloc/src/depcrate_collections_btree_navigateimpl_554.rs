// Generated macro for impl_554 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_554 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_554"}
// Dependencies: {}
impl < BorrowType : marker :: BorrowType , K , V > Handle < NodeRef < BorrowType , K , V , marker :: Internal > , marker :: Edge > { # [doc = " Given an internal edge handle, returns [`Result::Ok`] with a handle to the neighboring KV"] # [doc = " on the right side, which is either in the same internal node or in an ancestor node."] # [doc = " If the internal edge is the last one in the tree, returns [`Result::Err`] with the root node."] fn next_kv (self ,) -> Result < Handle < NodeRef < BorrowType , K , V , marker :: Internal > , marker :: KV > , NodeRef < BorrowType , K , V , marker :: Internal > , > { let mut edge = self ; loop { edge = match edge . right_kv () { Ok (internal_kv) => return Ok (internal_kv) , Err (last_edge) => match last_edge . into_node () . ascend () { Ok (parent_edge) => parent_edge , Err (root) => return Err (root) , } , } } } }
};
}

// Generated macro for impl_595 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_595 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_595"}
// Dependencies: {}
impl < BorrowType , K , V > NodeRef < BorrowType , K , V , marker :: Internal > { # [doc = " Exposes the data of an internal node."] # [doc = ""] # [doc = " Returns a raw ptr to avoid invalidating other references to this node."] fn as_internal_ptr (this : & Self) -> * mut InternalNode < K , V > { this . node . as_ptr () as * mut InternalNode < K , V > } }
};
}

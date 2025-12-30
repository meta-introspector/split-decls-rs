// Generated macro for impl_625 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_625 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_625"}
// Dependencies: {}
impl < BorrowType , K , V , NodeType , HandleType > PartialEq for Handle < NodeRef < BorrowType , K , V , NodeType > , HandleType > { fn eq (& self , other : & Self) -> bool { let Self { node , idx , _marker } = self ; node . eq (& other . node) && * idx == other . idx } }
};
}

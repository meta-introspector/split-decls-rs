// Generated macro for impl_599 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_599 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_599"}
// Dependencies: {}
impl < BorrowType , K , V , Type > NodeRef < BorrowType , K , V , Type > { # [doc = " Could be a public implementation of PartialEq, but only used in this module."] fn eq (& self , other : & Self) -> bool { let Self { node , height , _marker } = self ; if node . eq (& other . node) { debug_assert_eq ! (* height , other . height) ; true } else { false } } }
};
}

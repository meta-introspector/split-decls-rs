// Generated macro for Position (enum)
macro_rules! Depcrate_collections_btree_navigatePosition {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"Position"}
// Dependencies: {}
pub (super) enum Position < BorrowType , K , V > { Leaf (NodeRef < BorrowType , K , V , marker :: Leaf >) , Internal (NodeRef < BorrowType , K , V , marker :: Internal >) , InternalKV , }
};
}

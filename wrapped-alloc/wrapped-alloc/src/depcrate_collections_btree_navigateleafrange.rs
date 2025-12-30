// Generated macro for LeafRange (struct)
macro_rules! Depcrate_collections_btree_navigateLeafRange {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"LeafRange"}
// Dependencies: {}
pub (super) struct LeafRange < BorrowType , K , V > { front : Option < Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > > , back : Option < Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > > , }
};
}

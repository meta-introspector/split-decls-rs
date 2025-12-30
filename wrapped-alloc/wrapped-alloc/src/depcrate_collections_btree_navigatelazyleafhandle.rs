// Generated macro for LazyLeafHandle (enum)
macro_rules! Depcrate_collections_btree_navigateLazyLeafHandle {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"LazyLeafHandle"}
// Dependencies: {}
enum LazyLeafHandle < BorrowType , K , V > { Root (NodeRef < BorrowType , K , V , marker :: LeafOrInternal >) , Edge (Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge >) , }
};
}

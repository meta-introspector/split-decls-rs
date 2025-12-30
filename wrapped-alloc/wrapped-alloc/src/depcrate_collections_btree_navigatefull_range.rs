// Generated macro for full_range (function)
macro_rules! Depcrate_collections_btree_navigatefull_range {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"full_range"}
// Dependencies: {}
fn full_range < BorrowType : marker :: BorrowType , K , V > (root1 : NodeRef < BorrowType , K , V , marker :: LeafOrInternal > , root2 : NodeRef < BorrowType , K , V , marker :: LeafOrInternal > ,) -> LazyLeafRange < BorrowType , K , V > { LazyLeafRange { front : Some (LazyLeafHandle :: Root (root1)) , back : Some (LazyLeafHandle :: Root (root2)) , } }
};
}

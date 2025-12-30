// Generated macro for impl_547 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_547 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_547"}
// Dependencies: {}
impl < BorrowType : marker :: BorrowType , K , V > LazyLeafRange < BorrowType , K , V > { fn init_front (& mut self ,) -> Option < & mut Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > > { if let Some (LazyLeafHandle :: Root (root)) = & self . front { self . front = Some (LazyLeafHandle :: Edge (unsafe { ptr :: read (root) } . first_leaf_edge ())) ; } match & mut self . front { None => None , Some (LazyLeafHandle :: Edge (edge)) => Some (edge) , Some (LazyLeafHandle :: Root (_)) => unsafe { hint :: unreachable_unchecked () } , } } fn init_back (& mut self ,) -> Option < & mut Handle < NodeRef < BorrowType , K , V , marker :: Leaf > , marker :: Edge > > { if let Some (LazyLeafHandle :: Root (root)) = & self . back { self . back = Some (LazyLeafHandle :: Edge (unsafe { ptr :: read (root) } . last_leaf_edge ())) ; } match & mut self . back { None => None , Some (LazyLeafHandle :: Edge (edge)) => Some (edge) , Some (LazyLeafHandle :: Root (_)) => unsafe { hint :: unreachable_unchecked () } , } } }
};
}

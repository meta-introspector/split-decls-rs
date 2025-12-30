// Generated macro for impl_539 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_539 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_539"}
// Dependencies: {}
impl < BorrowType , K , V > LazyLeafHandle < BorrowType , K , V > { fn reborrow (& self) -> LazyLeafHandle < marker :: Immut < '_ > , K , V > { match self { LazyLeafHandle :: Root (root) => LazyLeafHandle :: Root (root . reborrow ()) , LazyLeafHandle :: Edge (edge) => LazyLeafHandle :: Edge (edge . reborrow ()) , } } }
};
}

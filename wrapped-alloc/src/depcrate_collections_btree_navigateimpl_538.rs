// Generated macro for impl_538 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_538 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_538"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > Clone for LazyLeafHandle < marker :: Immut < 'a > , K , V > { fn clone (& self) -> Self { match self { LazyLeafHandle :: Root (root) => LazyLeafHandle :: Root (* root) , LazyLeafHandle :: Edge (edge) => LazyLeafHandle :: Edge (* edge) , } } }
};
}

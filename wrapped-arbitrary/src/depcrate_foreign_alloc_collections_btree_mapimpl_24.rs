// Generated macro for impl_24 (impl)
macro_rules! Depcrate_foreign_alloc_collections_btree_mapimpl_24 {
() => {
// Module: crate::foreign::alloc::collections::btree_map
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a , K , V > Arbitrary < 'a > for BTreeMap < K , V > where K : Arbitrary < 'a > + Ord , V : Arbitrary < 'a > , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary_iter () ? . collect () } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { u . arbitrary_take_rest_iter () ? . collect () } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (0 , None) } }
};
}

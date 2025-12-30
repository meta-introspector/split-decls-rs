// Generated macro for impl_27 (impl)
macro_rules! Depcrate_foreign_alloc_collections_btree_setimpl_27 {
() => {
// Module: crate::foreign::alloc::collections::btree_set
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a , A > Arbitrary < 'a > for BTreeSet < A > where A : Arbitrary < 'a > + Ord , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary_iter () ? . collect () } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { u . arbitrary_take_rest_iter () ? . collect () } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (0 , None) } }
};
}

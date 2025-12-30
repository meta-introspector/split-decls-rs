// Generated macro for impl_30 (impl)
macro_rules! Depcrate_foreign_alloc_collections_linked_listimpl_30 {
() => {
// Module: crate::foreign::alloc::collections::linked_list
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a , A > Arbitrary < 'a > for LinkedList < A > where A : Arbitrary < 'a > , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary_iter () ? . collect () } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { u . arbitrary_take_rest_iter () ? . collect () } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (0 , None) } }
};
}

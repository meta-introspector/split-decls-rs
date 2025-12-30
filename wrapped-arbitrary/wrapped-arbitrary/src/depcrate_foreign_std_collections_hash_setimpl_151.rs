// Generated macro for impl_151 (impl)
macro_rules! Depcrate_foreign_std_collections_hash_setimpl_151 {
() => {
// Module: crate::foreign::std::collections::hash_set
// Provides: {"impl_151"}
// Dependencies: {}
impl < 'a , A , S > Arbitrary < 'a > for HashSet < A , S > where A : Arbitrary < 'a > + Eq + Hash , S : BuildHasher + Default , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary_iter () ? . collect () } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { u . arbitrary_take_rest_iter () ? . collect () } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (0 , None) } }
};
}

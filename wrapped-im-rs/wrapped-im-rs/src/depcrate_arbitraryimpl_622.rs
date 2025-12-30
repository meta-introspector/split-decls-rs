// Generated macro for impl_622 (impl)
macro_rules! Depcrate_arbitraryimpl_622 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_622"}
// Dependencies: {}
impl < 'a , A , S > Arbitrary < 'a > for HashSet < A , S > where A : Arbitrary < 'a > + Hash + Eq + Clone , S : BuildHasher + Default + 'static , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary_iter () ? . collect () } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { u . arbitrary_take_rest_iter () ? . collect () } fn size_hint (depth : usize) -> (usize , Option < usize >) { size_hint :: recursion_guard (depth , | depth | { size_hint :: and (< usize as Arbitrary > :: size_hint (depth) , (0 , None)) }) } }
};
}

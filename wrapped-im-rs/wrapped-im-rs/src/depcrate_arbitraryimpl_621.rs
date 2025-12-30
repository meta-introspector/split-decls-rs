// Generated macro for impl_621 (impl)
macro_rules! Depcrate_arbitraryimpl_621 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_621"}
// Dependencies: {}
impl < 'a , K , V , S > Arbitrary < 'a > for HashMap < K , V , S > where K : Arbitrary < 'a > + Hash + Eq + Clone , V : Arbitrary < 'a > + Clone , S : BuildHasher + Default + 'static , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary_iter () ? . collect () } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { u . arbitrary_take_rest_iter () ? . collect () } fn size_hint (depth : usize) -> (usize , Option < usize >) { size_hint :: recursion_guard (depth , | depth | { size_hint :: and (< usize as Arbitrary > :: size_hint (depth) , (0 , None)) }) } }
};
}

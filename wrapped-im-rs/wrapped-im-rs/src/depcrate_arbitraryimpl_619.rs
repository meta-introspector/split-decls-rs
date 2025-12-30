// Generated macro for impl_619 (impl)
macro_rules! Depcrate_arbitraryimpl_619 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_619"}
// Dependencies: {}
impl < 'a , K : Arbitrary < 'a > + Ord + Clone , V : Arbitrary < 'a > + Clone > Arbitrary < 'a > for OrdMap < K , V > { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary_iter () ? . collect () } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { u . arbitrary_take_rest_iter () ? . collect () } fn size_hint (depth : usize) -> (usize , Option < usize >) { size_hint :: recursion_guard (depth , | depth | { size_hint :: and (< usize as Arbitrary > :: size_hint (depth) , (0 , None)) }) } }
};
}

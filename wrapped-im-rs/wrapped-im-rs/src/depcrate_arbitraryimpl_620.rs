// Generated macro for impl_620 (impl)
macro_rules! Depcrate_arbitraryimpl_620 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_620"}
// Dependencies: {}
impl < 'a , A : Arbitrary < 'a > + Ord + Clone > Arbitrary < 'a > for OrdSet < A > { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary_iter () ? . collect () } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { u . arbitrary_take_rest_iter () ? . collect () } fn size_hint (depth : usize) -> (usize , Option < usize >) { size_hint :: recursion_guard (depth , | depth | { size_hint :: and (< usize as Arbitrary > :: size_hint (depth) , (0 , None)) }) } }
};
}

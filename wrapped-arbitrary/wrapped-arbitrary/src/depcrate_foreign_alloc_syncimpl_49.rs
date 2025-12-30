// Generated macro for impl_49 (impl)
macro_rules! Depcrate_foreign_alloc_syncimpl_49 {
() => {
// Module: crate::foreign::alloc::sync
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a , A > Arbitrary < 'a > for Arc < [A] > where A : Arbitrary < 'a > , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary_iter () ? . collect () } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { u . arbitrary_take_rest_iter () ? . collect () } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (0 , None) } }
};
}

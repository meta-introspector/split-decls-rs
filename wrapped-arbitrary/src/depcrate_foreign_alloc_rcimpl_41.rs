// Generated macro for impl_41 (impl)
macro_rules! Depcrate_foreign_alloc_rcimpl_41 {
() => {
// Module: crate::foreign::alloc::rc
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'a , A > Arbitrary < 'a > for Rc < [A] > where A : Arbitrary < 'a > , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary_iter () ? . collect () } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { u . arbitrary_take_rest_iter () ? . collect () } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (0 , None) } }
};
}

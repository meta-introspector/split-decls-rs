// Generated macro for impl_42 (impl)
macro_rules! Depcrate_foreign_alloc_rcimpl_42 {
() => {
// Module: crate::foreign::alloc::rc
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a > Arbitrary < 'a > for Rc < str > { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { < & str as Arbitrary > :: arbitrary (u) . map (Into :: into) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < & str as Arbitrary > :: size_hint (depth) } }
};
}

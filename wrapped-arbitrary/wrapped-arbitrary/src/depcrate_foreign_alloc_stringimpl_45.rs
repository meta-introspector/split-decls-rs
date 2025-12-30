// Generated macro for impl_45 (impl)
macro_rules! Depcrate_foreign_alloc_stringimpl_45 {
() => {
// Module: crate::foreign::alloc::string
// Provides: {"impl_45"}
// Dependencies: {}
impl < 'a > Arbitrary < 'a > for String { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { < & str as Arbitrary > :: arbitrary (u) . map (Into :: into) } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { < & str as Arbitrary > :: arbitrary_take_rest (u) . map (Into :: into) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < & str as Arbitrary > :: size_hint (depth) } }
};
}

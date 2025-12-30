// Generated macro for impl_50 (impl)
macro_rules! Depcrate_foreign_alloc_syncimpl_50 {
() => {
// Module: crate::foreign::alloc::sync
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'a > Arbitrary < 'a > for Arc < str > { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { < & str as Arbitrary > :: arbitrary (u) . map (Into :: into) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < & str as Arbitrary > :: size_hint (depth) } }
};
}

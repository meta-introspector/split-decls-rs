// Generated macro for impl_17 (impl)
macro_rules! Depcrate_foreign_alloc_boxedimpl_17 {
() => {
// Module: crate::foreign::alloc::boxed
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'a > Arbitrary < 'a > for Box < str > { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { < String as Arbitrary > :: arbitrary (u) . map (| x | x . into_boxed_str ()) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < String as Arbitrary > :: size_hint (depth) } }
};
}

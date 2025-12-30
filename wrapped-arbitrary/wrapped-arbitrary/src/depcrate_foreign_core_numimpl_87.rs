// Generated macro for impl_87 (impl)
macro_rules! Depcrate_foreign_core_numimpl_87 {
() => {
// Module: crate::foreign::core::num
// Provides: {"impl_87"}
// Dependencies: {}
impl < 'a > Arbitrary < 'a > for isize { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary :: < i64 > () . map (| x | x as isize) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < i64 as Arbitrary > :: size_hint (depth) } }
};
}

// Generated macro for impl_86 (impl)
macro_rules! Depcrate_foreign_core_numimpl_86 {
() => {
// Module: crate::foreign::core::num
// Provides: {"impl_86"}
// Dependencies: {}
impl < 'a > Arbitrary < 'a > for usize { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary :: < u64 > () . map (| x | x as usize) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < u64 as Arbitrary > :: size_hint (depth) } }
};
}

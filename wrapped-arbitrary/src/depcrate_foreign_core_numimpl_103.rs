// Generated macro for impl_103 (impl)
macro_rules! Depcrate_foreign_core_numimpl_103 {
() => {
// Module: crate::foreign::core::num
// Provides: {"impl_103"}
// Dependencies: {}
impl < 'a , A > Arbitrary < 'a > for Wrapping < A > where A : Arbitrary < 'a > , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Arbitrary :: arbitrary (u) . map (Wrapping) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { Self :: try_size_hint (depth) . unwrap_or_default () } # [inline] fn try_size_hint (depth : usize) -> Result < (usize , Option < usize >) , MaxRecursionReached > { < A as Arbitrary < 'a > > :: try_size_hint (depth) } }
};
}

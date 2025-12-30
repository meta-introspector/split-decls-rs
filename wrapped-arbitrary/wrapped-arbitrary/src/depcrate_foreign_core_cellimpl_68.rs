// Generated macro for impl_68 (impl)
macro_rules! Depcrate_foreign_core_cellimpl_68 {
() => {
// Module: crate::foreign::core::cell
// Provides: {"impl_68"}
// Dependencies: {}
impl < 'a , A > Arbitrary < 'a > for UnsafeCell < A > where A : Arbitrary < 'a > , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Arbitrary :: arbitrary (u) . map (Self :: new) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { Self :: try_size_hint (depth) . unwrap_or_default () } # [inline] fn try_size_hint (depth : usize) -> Result < (usize , Option < usize >) , MaxRecursionReached > { < A as Arbitrary < 'a > > :: try_size_hint (depth) } }
};
}

// Generated macro for impl_48 (impl)
macro_rules! Depcrate_foreign_alloc_syncimpl_48 {
() => {
// Module: crate::foreign::alloc::sync
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'a , A > Arbitrary < 'a > for Arc < A > where A : Arbitrary < 'a > , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Arbitrary :: arbitrary (u) . map (Self :: new) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { Self :: try_size_hint (depth) . unwrap_or_default () } # [inline] fn try_size_hint (depth : usize) -> Result < (usize , Option < usize >) , crate :: MaxRecursionReached > { size_hint :: try_recursion_guard (depth , < A as Arbitrary > :: try_size_hint) } }
};
}

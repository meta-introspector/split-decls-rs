// Generated macro for impl_15 (impl)
macro_rules! Depcrate_foreign_alloc_boxedimpl_15 {
() => {
// Module: crate::foreign::alloc::boxed
// Provides: {"impl_15"}
// Dependencies: {}
impl < 'a , A > Arbitrary < 'a > for Box < A > where A : Arbitrary < 'a > , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Arbitrary :: arbitrary (u) . map (Self :: new) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { Self :: try_size_hint (depth) . unwrap_or_default () } # [inline] fn try_size_hint (depth : usize) -> Result < (usize , Option < usize >) , crate :: MaxRecursionReached > { size_hint :: try_recursion_guard (depth , < A as Arbitrary > :: try_size_hint) } }
};
}

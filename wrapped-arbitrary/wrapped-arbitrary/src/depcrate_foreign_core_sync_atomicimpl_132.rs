// Generated macro for impl_132 (impl)
macro_rules! Depcrate_foreign_core_sync_atomicimpl_132 {
() => {
// Module: crate::foreign::core::sync::atomic
// Provides: {"impl_132"}
// Dependencies: {}
# [doc = " Returns zero, not an error, if this `Unstructured` [is empty][Unstructured::is_empty]."] impl < 'a > Arbitrary < 'a > for AtomicIsize { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Arbitrary :: arbitrary (u) . map (Self :: new) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < isize as Arbitrary < 'a > > :: size_hint (depth) } }
};
}

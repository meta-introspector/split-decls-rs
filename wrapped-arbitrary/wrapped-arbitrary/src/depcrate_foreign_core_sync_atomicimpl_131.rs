// Generated macro for impl_131 (impl)
macro_rules! Depcrate_foreign_core_sync_atomicimpl_131 {
() => {
// Module: crate::foreign::core::sync::atomic
// Provides: {"impl_131"}
// Dependencies: {}
# [doc = " Returns false, not an error, if this `Unstructured` [is empty][Unstructured::is_empty]."] impl < 'a > Arbitrary < 'a > for AtomicBool { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Arbitrary :: arbitrary (u) . map (Self :: new) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < bool as Arbitrary < 'a > > :: size_hint (depth) } }
};
}

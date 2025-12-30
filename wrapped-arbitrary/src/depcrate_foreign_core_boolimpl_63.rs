// Generated macro for impl_63 (impl)
macro_rules! Depcrate_foreign_core_boolimpl_63 {
() => {
// Module: crate::foreign::core::bool
// Provides: {"impl_63"}
// Dependencies: {}
# [doc = " Returns false, not an error, if this `Unstructured` [is empty][Unstructured::is_empty]."] impl < 'a > Arbitrary < 'a > for bool { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Ok (< u8 as Arbitrary < 'a > > :: arbitrary (u) ? & 1 == 1) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < u8 as Arbitrary < 'a > > :: size_hint (depth) } }
};
}

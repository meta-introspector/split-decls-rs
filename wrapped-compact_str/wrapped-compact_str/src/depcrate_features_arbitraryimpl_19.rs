// Generated macro for impl_19 (impl)
macro_rules! Depcrate_features_arbitraryimpl_19 {
() => {
// Module: crate::features::arbitrary
// Provides: {"impl_19"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "arbitrary")))] impl < 'a > Arbitrary < 'a > for CompactString { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { < & str as Arbitrary > :: arbitrary (u) . map (CompactString :: new) } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { < & str as Arbitrary > :: arbitrary_take_rest (u) . map (CompactString :: new) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < & str as Arbitrary > :: size_hint (depth) } }
};
}

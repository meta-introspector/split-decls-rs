// Generated macro for impl_315 (impl)
macro_rules! Depcrate_eventsimpl_315 {
() => {
// Module: crate::events
// Provides: {"impl_315"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < 'a > arbitrary :: Arbitrary < 'a > for BytesPI < 'a > { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { Ok (Self :: new (< & str > :: arbitrary (u) ?)) } fn size_hint (depth : usize) -> (usize , Option < usize >) { < & str as arbitrary :: Arbitrary > :: size_hint (depth) } }
};
}

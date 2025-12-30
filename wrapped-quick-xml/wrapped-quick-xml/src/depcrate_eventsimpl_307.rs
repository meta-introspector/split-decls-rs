// Generated macro for impl_307 (impl)
macro_rules! Depcrate_eventsimpl_307 {
() => {
// Module: crate::events
// Provides: {"impl_307"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < 'a > arbitrary :: Arbitrary < 'a > for BytesCData < 'a > { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { Ok (Self :: new (< & str > :: arbitrary (u) ?)) } fn size_hint (depth : usize) -> (usize , Option < usize >) { < & str as arbitrary :: Arbitrary > :: size_hint (depth) } }
};
}

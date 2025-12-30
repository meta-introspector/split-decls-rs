// Generated macro for impl_319 (impl)
macro_rules! Depcrate_eventsimpl_319 {
() => {
// Module: crate::events
// Provides: {"impl_319"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < 'a > arbitrary :: Arbitrary < 'a > for BytesDecl < 'a > { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { Ok (Self :: new (< & str > :: arbitrary (u) ? , Option :: < & str > :: arbitrary (u) ? , Option :: < & str > :: arbitrary (u) ? ,)) } fn size_hint (depth : usize) -> (usize , Option < usize >) { < & str as arbitrary :: Arbitrary > :: size_hint (depth) } }
};
}

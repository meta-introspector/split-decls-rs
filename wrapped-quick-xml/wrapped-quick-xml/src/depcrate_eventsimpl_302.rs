// Generated macro for impl_302 (impl)
macro_rules! Depcrate_eventsimpl_302 {
() => {
// Module: crate::events
// Provides: {"impl_302"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < 'a > arbitrary :: Arbitrary < 'a > for BytesText < 'a > { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { let s = < & str > :: arbitrary (u) ? ; if ! s . chars () . all (char :: is_alphanumeric) { return Err (arbitrary :: Error :: IncorrectFormat) ; } Ok (Self :: new (s)) } fn size_hint (depth : usize) -> (usize , Option < usize >) { < & str as arbitrary :: Arbitrary > :: size_hint (depth) } }
};
}

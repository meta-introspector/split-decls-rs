// Generated macro for impl_291 (impl)
macro_rules! Depcrate_eventsimpl_291 {
() => {
// Module: crate::events
// Provides: {"impl_291"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < 'a > arbitrary :: Arbitrary < 'a > for BytesStart < 'a > { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { let s = < & str > :: arbitrary (u) ? ; if s . is_empty () || ! s . chars () . all (char :: is_alphanumeric) { return Err (arbitrary :: Error :: IncorrectFormat) ; } let mut result = Self :: new (s) ; result . extend_attributes (Vec :: < (& str , & str) > :: arbitrary (u) ?) ; Ok (result) } fn size_hint (depth : usize) -> (usize , Option < usize >) { < & str as arbitrary :: Arbitrary > :: size_hint (depth) } }
};
}

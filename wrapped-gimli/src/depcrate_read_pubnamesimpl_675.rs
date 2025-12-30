// Generated macro for impl_675 (impl)
macro_rules! Depcrate_read_pubnamesimpl_675 {
() => {
// Module: crate::read::pubnames
// Provides: {"impl_675"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for PubNamesEntryIter < R > { type Item = PubNamesEntry < R > ; type Error = crate :: read :: Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { self . 0 . next () } }
};
}

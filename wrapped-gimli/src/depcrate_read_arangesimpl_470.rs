// Generated macro for impl_470 (impl)
macro_rules! Depcrate_read_arangesimpl_470 {
() => {
// Module: crate::read::aranges
// Provides: {"impl_470"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for ArangeEntryIter < R > { type Item = ArangeEntry ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { ArangeEntryIter :: next (self) } }
};
}

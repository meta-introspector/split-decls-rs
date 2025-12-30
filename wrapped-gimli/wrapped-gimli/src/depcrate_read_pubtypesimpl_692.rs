// Generated macro for impl_692 (impl)
macro_rules! Depcrate_read_pubtypesimpl_692 {
() => {
// Module: crate::read::pubtypes
// Provides: {"impl_692"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for PubTypesEntryIter < R > { type Item = PubTypesEntry < R > ; type Error = crate :: read :: Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { self . 0 . next () } }
};
}

// Generated macro for impl_581 (impl)
macro_rules! Depcrate_read_loclistsimpl_581 {
() => {
// Module: crate::read::loclists
// Provides: {"impl_581"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for RawLocListIter < R > { type Item = RawLocListEntry < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { RawLocListIter :: next (self) } }
};
}

// Generated macro for impl_584 (impl)
macro_rules! Depcrate_read_loclistsimpl_584 {
() => {
// Module: crate::read::loclists
// Provides: {"impl_584"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for LocListIter < R > { type Item = LocationListEntry < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { LocListIter :: next (self) } }
};
}

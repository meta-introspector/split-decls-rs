// Generated macro for impl_723 (impl)
macro_rules! Depcrate_read_rnglistsimpl_723 {
() => {
// Module: crate::read::rnglists
// Provides: {"impl_723"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for RngListIter < R > { type Item = Range ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { RngListIter :: next (self) } }
};
}

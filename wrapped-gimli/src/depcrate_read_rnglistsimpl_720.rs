// Generated macro for impl_720 (impl)
macro_rules! Depcrate_read_rnglistsimpl_720 {
() => {
// Module: crate::read::rnglists
// Provides: {"impl_720"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for RawRngListIter < R > { type Item = RawRngListEntry < R :: Offset > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { RawRngListIter :: next (self) } }
};
}

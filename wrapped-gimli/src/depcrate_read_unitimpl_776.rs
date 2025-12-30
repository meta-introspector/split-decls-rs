// Generated macro for impl_776 (impl)
macro_rules! Depcrate_read_unitimpl_776 {
() => {
// Module: crate::read::unit
// Provides: {"impl_776"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for DebugInfoUnitHeadersIter < R > { type Item = UnitHeader < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { DebugInfoUnitHeadersIter :: next (self) } }
};
}

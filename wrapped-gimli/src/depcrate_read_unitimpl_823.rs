// Generated macro for impl_823 (impl)
macro_rules! Depcrate_read_unitimpl_823 {
() => {
// Module: crate::read::unit
// Provides: {"impl_823"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for DebugTypesUnitHeadersIter < R > { type Item = UnitHeader < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { DebugTypesUnitHeadersIter :: next (self) } }
};
}

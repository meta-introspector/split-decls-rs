// Generated macro for impl_652 (impl)
macro_rules! Depcrate_read_opimpl_652 {
() => {
// Module: crate::read::op
// Provides: {"impl_652"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for OperationIter < R > { type Item = Operation < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { OperationIter :: next (self) } }
};
}

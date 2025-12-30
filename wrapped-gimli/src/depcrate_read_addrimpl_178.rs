// Generated macro for impl_178 (impl)
macro_rules! Depcrate_read_addrimpl_178 {
() => {
// Module: crate::read::addr
// Provides: {"impl_178"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for AddrHeaderIter < R > { type Item = AddrHeader < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { AddrHeaderIter :: next (self) } }
};
}

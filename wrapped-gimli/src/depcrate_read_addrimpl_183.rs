// Generated macro for impl_183 (impl)
macro_rules! Depcrate_read_addrimpl_183 {
() => {
// Module: crate::read::addr
// Provides: {"impl_183"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for AddrEntryIter < R > { type Item = u64 ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { AddrEntryIter :: next (self) } }
};
}

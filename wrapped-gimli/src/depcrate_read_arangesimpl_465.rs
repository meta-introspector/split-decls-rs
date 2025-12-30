// Generated macro for impl_465 (impl)
macro_rules! Depcrate_read_arangesimpl_465 {
() => {
// Module: crate::read::aranges
// Provides: {"impl_465"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for ArangeHeaderIter < R > { type Item = ArangeHeader < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { ArangeHeaderIter :: next (self) } }
};
}

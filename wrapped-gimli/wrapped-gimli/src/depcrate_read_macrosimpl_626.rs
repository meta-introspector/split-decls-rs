// Generated macro for impl_626 (impl)
macro_rules! Depcrate_read_macrosimpl_626 {
() => {
// Module: crate::read::macros
// Provides: {"impl_626"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for MacroIter < R > { type Item = MacroEntry < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Error > { MacroIter :: next (self) } }
};
}

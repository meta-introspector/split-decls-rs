// Generated macro for impl_327 (impl)
macro_rules! Depcrate_read_dwarfimpl_327 {
() => {
// Module: crate::read::dwarf
// Provides: {"impl_327"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < R : Reader > fallible_iterator :: FallibleIterator for RangeIter < R > { type Item = Range ; type Error = Error ; # [inline] fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { RangeIter :: next (self) } }
};
}

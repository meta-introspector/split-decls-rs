// Generated macro for impl_802 (impl)
macro_rules! Depcrate_read_unitimpl_802 {
() => {
// Module: crate::read::unit
// Provides: {"impl_802"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < 'abbrev , 'entry , 'unit , R : Reader > fallible_iterator :: FallibleIterator for AttrsIter < 'abbrev , 'entry , 'unit , R > { type Item = Attribute < R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { AttrsIter :: next (self) } }
};
}

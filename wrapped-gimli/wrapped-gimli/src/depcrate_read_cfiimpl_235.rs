// Generated macro for impl_235 (impl)
macro_rules! Depcrate_read_cfiimpl_235 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_235"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < 'bases , Section , R > fallible_iterator :: FallibleIterator for CfiEntriesIter < 'bases , Section , R > where R : Reader , Section : UnwindSection < R > , { type Item = CieOrFde < 'bases , Section , R > ; type Error = Error ; fn next (& mut self) -> :: core :: result :: Result < Option < Self :: Item > , Self :: Error > { CfiEntriesIter :: next (self) } }
};
}

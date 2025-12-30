// Generated macro for impl_1061 (impl)
macro_rules! Depcrate_util_searchimpl_1061 {
() => {
// Module: crate::util::search
// Provides: {"impl_1061"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a > DoubleEndedIterator for PatternSetIter < 'a > { fn next_back (& mut self) -> Option < PatternID > { while let Some ((index , & yes)) = self . it . next_back () { if yes { return Some (PatternID :: new_unchecked (index)) ; } } None } }
};
}

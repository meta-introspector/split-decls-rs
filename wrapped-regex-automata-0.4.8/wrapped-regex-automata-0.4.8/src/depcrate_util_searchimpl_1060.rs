// Generated macro for impl_1060 (impl)
macro_rules! Depcrate_util_searchimpl_1060 {
() => {
// Module: crate::util::search
// Provides: {"impl_1060"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a > Iterator for PatternSetIter < 'a > { type Item = PatternID ; fn next (& mut self) -> Option < PatternID > { while let Some ((index , & yes)) = self . it . next () { if yes { return Some (PatternID :: new_unchecked (index)) ; } } None } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}

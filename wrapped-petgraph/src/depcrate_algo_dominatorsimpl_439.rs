// Generated macro for impl_439 (impl)
macro_rules! Depcrate_algo_dominatorsimpl_439 {
() => {
// Module: crate::algo::dominators
// Provides: {"impl_439"}
// Dependencies: {}
impl < 'a , N > Iterator for DominatedByIter < 'a , N > where N : 'a + Copy + Eq + Hash , { type Item = N ; fn next (& mut self) -> Option < Self :: Item > { for (dominator , dominated) in self . iter . by_ref () { if dominated == & self . node && dominated != dominator { return Some (* dominator) ; } } None } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
};
}

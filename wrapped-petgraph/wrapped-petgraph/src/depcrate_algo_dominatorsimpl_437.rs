// Generated macro for impl_437 (impl)
macro_rules! Depcrate_algo_dominatorsimpl_437 {
() => {
// Module: crate::algo::dominators
// Provides: {"impl_437"}
// Dependencies: {}
impl < 'a , N > Iterator for DominatorsIter < 'a , N > where N : 'a + Copy + Eq + Hash , { type Item = N ; fn next (& mut self) -> Option < Self :: Item > { let next = self . node . take () ; if let Some (next) = next { self . node = self . dominators . immediate_dominator (next) ; } next } }
};
}

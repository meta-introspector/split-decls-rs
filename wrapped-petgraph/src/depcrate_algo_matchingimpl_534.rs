// Generated macro for impl_534 (impl)
macro_rules! Depcrate_algo_matchingimpl_534 {
() => {
// Module: crate::algo::matching
// Provides: {"impl_534"}
// Dependencies: {}
impl < G > Iterator for MatchedNodes < '_ , G > where G : NodeIndexable , { type Item = G :: NodeId ; fn next (& mut self) -> Option < Self :: Item > { while self . current != self . mate . len () { let current = self . current ; self . current += 1 ; if self . mate [current] . is_some () { return Some (self . graph . from_index (current)) ; } } None } }
};
}

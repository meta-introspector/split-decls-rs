// Generated macro for impl_536 (impl)
macro_rules! Depcrate_algo_matchingimpl_536 {
() => {
// Module: crate::algo::matching
// Provides: {"impl_536"}
// Dependencies: {}
impl < G > Iterator for MatchedEdges < '_ , G > where G : NodeIndexable , { type Item = (G :: NodeId , G :: NodeId) ; fn next (& mut self) -> Option < Self :: Item > { while self . current != self . mate . len () { let current = self . current ; self . current += 1 ; if let Some (mate) = self . mate [current] { if self . graph . to_index (mate) > current { let this = self . graph . from_index (current) ; return Some ((this , mate)) ; } } } None } }
};
}

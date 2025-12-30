// Generated macro for impl_239 (impl)
macro_rules! Depcrate_topo_optimizerimpl_239 {
() => {
// Module: crate::topo::optimizer
// Provides: {"impl_239"}
// Dependencies: {}
impl < 'a > RankOptimizer < 'a > { pub fn new (dag : & 'a mut DAG) -> Self { Self { dag } } pub fn try_to_sink_node (& mut self , node : NodeHandle) -> bool { let backs = self . dag . predecessors (node) ; let fwds = self . dag . successors (node) ; if backs . len () > fwds . len () || backs . len () + fwds . len () == 0 { return false ; } let curr_rank = self . dag . level (node) ; let mut highest_next = self . dag . len () ; for elem in fwds { let next_rank = self . dag . level (* elem) ; highest_next = highest_next . min (next_rank) ; } if highest_next > curr_rank + 1 { self . dag . update_node_rank_level (node , highest_next - 1 , None) ; return true ; } false } pub fn optimize (& mut self) { self . dag . verify () ; # [cfg (feature = "log")] log :: info ! ("Optimizing the ranks.") ; # [cfg (feature = "log")] let mut cnt = 0 ; # [cfg (feature = "log")] let mut iter = 0 ; loop { let mut c = 0 ; for node in self . dag . iter () { if self . try_to_sink_node (node) { c += 1 ; } } # [cfg (feature = "log")] { cnt += c ; iter += 1 ; } if c == 0 { break ; } } # [cfg (feature = "log")] log :: info ! ("Sank {} nodes in {} iteration." , cnt , iter) ; } }
};
}

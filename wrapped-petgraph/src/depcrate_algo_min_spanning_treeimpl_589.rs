// Generated macro for impl_589 (impl)
macro_rules! Depcrate_algo_min_spanning_treeimpl_589 {
() => {
// Module: crate::algo::min_spanning_tree
// Provides: {"impl_589"}
// Dependencies: {}
impl < G > Iterator for MinSpanningTree < G > where G : IntoNodeReferences + NodeIndexable , G :: NodeWeight : Clone , G :: EdgeWeight : PartialOrd , { type Item = Element < G :: NodeWeight , G :: EdgeWeight > ; fn next (& mut self) -> Option < Self :: Item > { let g = self . graph ; if let Some (ref mut iter) = self . node_ids { if let Some (node) = iter . next () { self . node_map . insert (g . to_index (node . id ()) , self . node_count) ; self . node_count += 1 ; return Some (Element :: Node { weight : node . weight () . clone () , }) ; } } self . node_ids = None ; while let Some (MinScored (score , (a , b))) = self . sort_edges . pop () { let (a_index , b_index) = (g . to_index (a) , g . to_index (b)) ; if self . subgraphs . union (a_index , b_index) { let (& a_order , & b_order) = match (self . node_map . get (& a_index) , self . node_map . get (& b_index)) { (Some (a_id) , Some (b_id)) => (a_id , b_id) , _ => panic ! ("Edge references unknown node") , } ; return Some (Element :: Edge { source : a_order , target : b_order , weight : score , }) ; } } None } }
};
}

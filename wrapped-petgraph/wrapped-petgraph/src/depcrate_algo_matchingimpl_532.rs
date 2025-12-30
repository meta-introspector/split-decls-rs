// Generated macro for impl_532 (impl)
macro_rules! Depcrate_algo_matchingimpl_532 {
() => {
// Module: crate::algo::matching
// Provides: {"impl_532"}
// Dependencies: {}
impl < G : NodeIndexable > WithDummy for G { fn dummy_idx (& self) -> usize { self . node_bound () } fn try_from_index (& self , i : usize) -> Option < Self :: NodeId > { if i != self . dummy_idx () { Some (self . from_index (i)) } else { None } } }
};
}

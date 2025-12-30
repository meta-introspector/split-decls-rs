// Generated macro for WithDummy (trait)
macro_rules! Depcrate_algo_matchingWithDummy {
() => {
// Module: crate::algo::matching
// Provides: {"WithDummy"}
// Dependencies: {}
trait WithDummy : NodeIndexable { fn dummy_idx (& self) -> usize ; # [doc = " Convert `i` to a node index, returns None for the dummy node"] fn try_from_index (& self , i : usize) -> Option < Self :: NodeId > ; }
};
}

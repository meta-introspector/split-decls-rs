// Generated macro for impl_1008 (impl)
macro_rules! Depcrate_strategy_unionsimpl_1008 {
() => {
// Module: crate::strategy::unions
// Provides: {"impl_1008"}
// Dependencies: {}
impl < T : Strategy > Strategy for Union < T > { type Tree = UnionValueTree < T > ; type Value = T :: Value ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { fn extract_weight < V > (& (w , _) : & WA < V >) -> u32 { w } let pick = pick_weighted (runner , self . options . iter () . map (extract_weight :: < T >) , self . options . iter () . map (extract_weight :: < T >) ,) ; let mut options = Vec :: with_capacity (pick) ; for option in & self . options [0 .. pick] { options . push (LazyValueTree :: new (Arc :: clone (& option . 1) , runner)) ; } options . push (LazyValueTree :: new_initialized (self . options [pick] . 1 . new_tree (runner) ? ,)) ; Ok (UnionValueTree { options , pick , min_pick : 0 , prev_pick : None , }) } }
};
}

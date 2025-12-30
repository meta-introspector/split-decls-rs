// Generated macro for impl_62 (impl)
macro_rules! Depcrate_iterators_flat_pairsimpl_62 {
() => {
// Module: crate::iterators::flat_pairs
// Provides: {"impl_62"}
// Dependencies: {}
impl < R : RuleType > DoubleEndedIterator for FlatPairs < '_ , R > { fn next_back (& mut self) -> Option < Self :: Item > { if self . end <= self . start { return None ; } self . next_start_from_end () ; let pair = pair :: new (Rc :: clone (& self . queue) , self . input , Rc :: clone (& self . line_index) , self . end ,) ; Some (pair) } }
};
}

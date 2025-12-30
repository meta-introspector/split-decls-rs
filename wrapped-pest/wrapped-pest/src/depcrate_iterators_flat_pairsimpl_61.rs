// Generated macro for impl_61 (impl)
macro_rules! Depcrate_iterators_flat_pairsimpl_61 {
() => {
// Module: crate::iterators::flat_pairs
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'i , R : RuleType > Iterator for FlatPairs < 'i , R > { type Item = Pair < 'i , R > ; fn next (& mut self) -> Option < Self :: Item > { if self . start >= self . end { return None ; } let pair = pair :: new (Rc :: clone (& self . queue) , self . input , Rc :: clone (& self . line_index) , self . start ,) ; self . next_start () ; Some (pair) } fn size_hint (& self) -> (usize , Option < usize >) { let len = < Self as ExactSizeIterator > :: len (self) ; (len , Some (len)) } }
};
}

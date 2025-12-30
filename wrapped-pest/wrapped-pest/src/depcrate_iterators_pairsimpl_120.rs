// Generated macro for impl_120 (impl)
macro_rules! Depcrate_iterators_pairsimpl_120 {
() => {
// Module: crate::iterators::pairs
// Provides: {"impl_120"}
// Dependencies: {}
impl < 'i , R : RuleType > Iterator for Pairs < 'i , R > { type Item = Pair < 'i , R > ; fn next (& mut self) -> Option < Self :: Item > { let pair = self . peek () ? ; self . start = self . pair () + 1 ; self . pairs_count -= 1 ; Some (pair) } fn size_hint (& self) -> (usize , Option < usize >) { let len = < Self as ExactSizeIterator > :: len (self) ; (len , Some (len)) } }
};
}

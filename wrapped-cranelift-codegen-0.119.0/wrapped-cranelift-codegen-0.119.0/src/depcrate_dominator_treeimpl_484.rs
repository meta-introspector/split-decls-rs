// Generated macro for impl_484 (impl)
macro_rules! Depcrate_dominator_treeimpl_484 {
() => {
// Module: crate::dominator_tree
// Provides: {"impl_484"}
// Dependencies: {}
impl < 'a > Iterator for ChildIter < 'a > { type Item = Block ; fn next (& mut self) -> Option < Block > { let n = self . next . expand () ; if let Some (block) = n { self . next = self . dtpo . nodes [block] . sibling ; } n } }
};
}

// Generated macro for impl_148 (impl)
macro_rules! Depcrate_visit_filterimpl_148 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_148"}
// Dependencies: {}
impl < I , F > Iterator for NodeFilteredNeighbors < '_ , I , F > where I : Iterator , I :: Item : Copy , F : FilterNode < I :: Item > , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { let f = self . f ; if ! self . include_source { None } else { self . iter . find (move | & target | f . include_node (target)) } } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
};
}

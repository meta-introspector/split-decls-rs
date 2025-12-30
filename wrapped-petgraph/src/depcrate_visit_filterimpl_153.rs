// Generated macro for impl_153 (impl)
macro_rules! Depcrate_visit_filterimpl_153 {
() => {
// Module: crate::visit::filter
// Provides: {"impl_153"}
// Dependencies: {}
impl < I , F > Iterator for NodeFilteredNodes < '_ , I , F > where I : Iterator , I :: Item : Copy + NodeRef , F : FilterNode < < I :: Item as NodeRef > :: NodeId > , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { let f = self . f ; if ! self . include_source { None } else { self . iter . find (move | & target | f . include_node (target . id ())) } } fn size_hint (& self) -> (usize , Option < usize >) { let (_ , upper) = self . iter . size_hint () ; (0 , upper) } }
};
}

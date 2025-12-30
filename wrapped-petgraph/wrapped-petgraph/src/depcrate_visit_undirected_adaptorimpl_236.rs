// Generated macro for impl_236 (impl)
macro_rules! Depcrate_visit_undirected_adaptorimpl_236 {
() => {
// Module: crate::visit::undirected_adaptor
// Provides: {"impl_236"}
// Dependencies: {}
impl < I > Iterator for MaybeReversedEdgeReferences < I > where I : Iterator , I :: Item : EdgeRef , { type Item = MaybeReversedEdgeReference < I :: Item > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| x | MaybeReversedEdgeReference { inner : x , reversed : false , }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}

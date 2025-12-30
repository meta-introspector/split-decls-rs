// Generated macro for impl_232 (impl)
macro_rules! Depcrate_visit_undirected_adaptorimpl_232 {
() => {
// Module: crate::visit::undirected_adaptor
// Provides: {"impl_232"}
// Dependencies: {}
impl < I > Iterator for MaybeReversedEdges < I > where I : Iterator , I :: Item : EdgeRef , { type Item = MaybeReversedEdgeReference < I :: Item > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| x | MaybeReversedEdgeReference { inner : x , reversed : self . reversed , }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}

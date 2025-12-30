// Generated macro for impl_212 (impl)
macro_rules! Depcrate_visit_reversedimpl_212 {
() => {
// Module: crate::visit::reversed
// Provides: {"impl_212"}
// Dependencies: {}
impl < I > Iterator for ReversedEdgeReferences < I > where I : Iterator , I :: Item : EdgeRef , { type Item = ReversedEdgeReference < I :: Item > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (ReversedEdgeReference) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}

// Generated macro for impl_83 (impl)
macro_rules! Depcrate_apiimpl_83 {
() => {
// Module: crate::api
// Provides: {"impl_83"}
// Dependencies: {}
impl < L : Language > Iterator for Preorder < L > { type Item = WalkEvent < SyntaxNode < L > > ; fn next (& mut self) -> Option < Self :: Item > { self . raw . next () . map (| it | it . map (SyntaxNode :: from)) } }
};
}

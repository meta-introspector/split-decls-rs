// Generated macro for impl_76 (impl)
macro_rules! Depcrate_apiimpl_76 {
() => {
// Module: crate::api
// Provides: {"impl_76"}
// Dependencies: {}
impl < L : Language > Iterator for SyntaxNodeChildren < L > { type Item = SyntaxNode < L > ; fn next (& mut self) -> Option < Self :: Item > { self . raw . next () . map (SyntaxNode :: from) } }
};
}

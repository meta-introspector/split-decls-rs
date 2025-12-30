// Generated macro for impl_79 (impl)
macro_rules! Depcrate_apiimpl_79 {
() => {
// Module: crate::api
// Provides: {"impl_79"}
// Dependencies: {}
impl < L : Language > Iterator for SyntaxElementChildren < L > { type Item = SyntaxElement < L > ; fn next (& mut self) -> Option < Self :: Item > { self . raw . next () . map (NodeOrToken :: from) } }
};
}

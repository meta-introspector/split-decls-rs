// Generated macro for impl_86 (impl)
macro_rules! Depcrate_apiimpl_86 {
() => {
// Module: crate::api
// Provides: {"impl_86"}
// Dependencies: {}
impl < L : Language > Iterator for PreorderWithTokens < L > { type Item = WalkEvent < SyntaxElement < L > > ; fn next (& mut self) -> Option < Self :: Item > { self . raw . next () . map (| it | it . map (SyntaxElement :: from)) } }
};
}

// Generated macro for impl_134 (impl)
macro_rules! Depcrate_nfaimpl_134 {
() => {
// Module: crate::nfa
// Provides: {"impl_134"}
// Dependencies: {}
impl < 'a > Iterator for CaptureNames < 'a > { type Item = Option < & 'a str > ; fn next (& mut self) -> Option < Option < & 'a str > > { self . it . next () . map (| n | n . as_deref ()) } }
};
}

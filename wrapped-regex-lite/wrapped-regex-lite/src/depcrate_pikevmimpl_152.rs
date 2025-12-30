// Generated macro for impl_152 (impl)
macro_rules! Depcrate_pikevmimpl_152 {
() => {
// Module: crate::pikevm
// Provides: {"impl_152"}
// Dependencies: {}
impl < 'r , 'h > Iterator for CapturesMatches < 'r , 'h > { type Item = Vec < Option < NonMaxUsize > > ; fn next (& mut self) -> Option < Vec < Option < NonMaxUsize > > > { self . it . next () ? ; Some (self . it . slots . clone ()) } }
};
}

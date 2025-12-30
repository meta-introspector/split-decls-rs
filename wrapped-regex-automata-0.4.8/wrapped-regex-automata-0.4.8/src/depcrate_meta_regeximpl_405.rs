// Generated macro for impl_405 (impl)
macro_rules! Depcrate_meta_regeximpl_405 {
() => {
// Module: crate::meta::regex
// Provides: {"impl_405"}
// Dependencies: {}
impl < 'r , 'h > Iterator for Split < 'r , 'h > { type Item = Span ; fn next (& mut self) -> Option < Span > { match self . finder . next () { None => { let len = self . finder . it . input () . haystack () . len () ; if self . last > len { None } else { let span = Span :: from (self . last .. len) ; self . last = len + 1 ; Some (span) } } Some (m) => { let span = Span :: from (self . last .. m . start ()) ; self . last = m . end () ; Some (span) } } } }
};
}

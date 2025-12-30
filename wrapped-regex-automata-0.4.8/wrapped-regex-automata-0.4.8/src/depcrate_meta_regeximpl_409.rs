// Generated macro for impl_409 (impl)
macro_rules! Depcrate_meta_regeximpl_409 {
() => {
// Module: crate::meta::regex
// Provides: {"impl_409"}
// Dependencies: {}
impl < 'r , 'h > Iterator for SplitN < 'r , 'h > { type Item = Span ; fn next (& mut self) -> Option < Span > { if self . limit == 0 { return None ; } self . limit -= 1 ; if self . limit > 0 { return self . splits . next () ; } let len = self . splits . finder . it . input () . haystack () . len () ; if self . splits . last > len { None } else { Some (Span :: from (self . splits . last .. len)) } } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (self . limit)) } }
};
}

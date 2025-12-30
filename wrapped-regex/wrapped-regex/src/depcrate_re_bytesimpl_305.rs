// Generated macro for impl_305 (impl)
macro_rules! Depcrate_re_bytesimpl_305 {
() => {
// Module: crate::re_bytes
// Provides: {"impl_305"}
// Dependencies: {}
impl < 'r , 't > Iterator for Splits < 'r , 't > { type Item = & 't [u8] ; fn next (& mut self) -> Option < & 't [u8] > { let text = self . finder . 0 . text () ; match self . finder . next () { None => { if self . last >= text . len () { None } else { let s = & text [self . last ..] ; self . last = text . len () ; Some (s) } } Some ((s , e)) => { let matched = & text [self . last .. s] ; self . last = e ; Some (matched) } } } }
};
}

// Generated macro for impl_307 (impl)
macro_rules! Depcrate_re_bytesimpl_307 {
() => {
// Module: crate::re_bytes
// Provides: {"impl_307"}
// Dependencies: {}
impl < 'r , 't > Iterator for SplitsN < 'r , 't > { type Item = & 't [u8] ; fn next (& mut self) -> Option < & 't [u8] > { if self . n == 0 { return None } self . n -= 1 ; if self . n == 0 { let text = self . splits . finder . 0 . text () ; Some (& text [self . splits . last ..]) } else { self . splits . next () } } }
};
}

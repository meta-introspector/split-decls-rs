// Generated macro for impl_383 (impl)
macro_rules! Depcrate_re_unicodeimpl_383 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_383"}
// Dependencies: {}
impl < 'r , 't > Iterator for RegexSplitsN < 'r , 't > { type Item = & 't str ; fn next (& mut self) -> Option < & 't str > { if self . n == 0 { return None } self . n -= 1 ; if self . n == 0 { let text = self . splits . finder . text () ; Some (& text [self . splits . last ..]) } else { self . splits . next () } } }
};
}

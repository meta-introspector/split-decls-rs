// Generated macro for impl_381 (impl)
macro_rules! Depcrate_re_unicodeimpl_381 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_381"}
// Dependencies: {}
impl < 'r , 't > Iterator for RegexSplits < 'r , 't > { type Item = & 't str ; fn next (& mut self) -> Option < & 't str > { let text = self . finder . text () ; match self . finder . next () { None => { if self . last >= text . len () { None } else { let s = & text [self . last ..] ; self . last = text . len () ; Some (s) } } Some ((s , e)) => { let matched = & text [self . last .. s] ; self . last = e ; Some (matched) } } } }
};
}

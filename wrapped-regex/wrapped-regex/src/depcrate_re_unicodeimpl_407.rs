// Generated macro for impl_407 (impl)
macro_rules! Depcrate_re_unicodeimpl_407 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_407"}
// Dependencies: {}
impl < 'r , 't > Iterator for FindMatches < 'r , 't > { type Item = (usize , usize) ; fn next (& mut self) -> Option < (usize , usize) > { match self . 0 { FindMatchesInner :: Dynamic (ref mut it) => it . next () , FindMatchesInner :: Plugin (ref mut it) => it . next () , } } }
};
}

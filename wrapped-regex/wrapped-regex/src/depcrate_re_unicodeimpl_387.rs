// Generated macro for impl_387 (impl)
macro_rules! Depcrate_re_unicodeimpl_387 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_387"}
// Dependencies: {}
impl < 'n > Iterator for NamedGroupsIter < 'n > { type Item = (& 'n str , usize) ; fn next (& mut self) -> Option < Self :: Item > { match * self { NamedGroupsIter :: Plugin (ref mut it) => it . next () . map (| & v | v) , NamedGroupsIter :: Dynamic (ref mut it) => { it . next () . map (| (s , i) | (s . as_ref () , * i)) } } } }
};
}

// Generated macro for impl_400 (impl)
macro_rules! Depcrate_re_unicodeimpl_400 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_400"}
// Dependencies: {}
impl < 'c > Iterator for SubCapturesNamed < 'c > { type Item = (& 'c str , Option < & 'c str >) ; fn next (& mut self) -> Option < (& 'c str , Option < & 'c str >) > { self . names . next () . map (| (name , pos) | (name , self . caps . at (pos))) } }
};
}

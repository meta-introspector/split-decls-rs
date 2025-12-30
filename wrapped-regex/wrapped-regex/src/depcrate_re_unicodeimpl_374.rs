// Generated macro for impl_374 (impl)
macro_rules! Depcrate_re_unicodeimpl_374 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_374"}
// Dependencies: {}
impl < 'r > Iterator for CaptureNames < 'r > { type Item = Option < & 'r str > ; fn next (& mut self) -> Option < Option < & 'r str > > { match self . 0 { _CaptureNames :: Plugin (ref mut i) => i . next () . cloned () , _CaptureNames :: Dynamic (ref mut i) => { i . next () . as_ref () . map (| o | o . as_ref () . map (| s | s . as_ref ())) } } } fn size_hint (& self) -> (usize , Option < usize >) { match self . 0 { _CaptureNames :: Plugin (ref i) => i . size_hint () , _CaptureNames :: Dynamic (ref i) => i . size_hint () , } } }
};
}

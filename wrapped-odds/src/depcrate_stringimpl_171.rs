// Generated macro for impl_171 (impl)
macro_rules! Depcrate_stringimpl_171 {
() => {
// Module: crate::string
// Provides: {"impl_171"}
// Dependencies: {}
impl < 'a > Iterator for Suffixes < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < & 'a str > { self . iter . next () . map (| (i , _) | & self . s [i ..]) } }
};
}

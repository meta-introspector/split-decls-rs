// Generated macro for impl_169 (impl)
macro_rules! Depcrate_stringimpl_169 {
() => {
// Module: crate::string
// Provides: {"impl_169"}
// Dependencies: {}
impl < 'a > Iterator for Prefixes < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < & 'a str > { self . iter . next () . map (| (i , ch) | & self . s [.. i + ch . len_utf8 ()]) } }
};
}

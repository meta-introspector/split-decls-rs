// Generated macro for impl_128 (impl)
macro_rules! Depcrate_regex_stringimpl_128 {
() => {
// Module: crate::regex::string
// Provides: {"impl_128"}
// Dependencies: {}
impl < 'r , 'h > Iterator for Split < 'r , 'h > { type Item = & 'h str ; # [inline] fn next (& mut self) -> Option < & 'h str > { self . it . next () . map (| span | & self . haystack [span]) } }
};
}

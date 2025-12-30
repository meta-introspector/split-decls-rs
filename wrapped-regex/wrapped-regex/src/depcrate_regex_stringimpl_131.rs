// Generated macro for impl_131 (impl)
macro_rules! Depcrate_regex_stringimpl_131 {
() => {
// Module: crate::regex::string
// Provides: {"impl_131"}
// Dependencies: {}
impl < 'r , 'h > Iterator for SplitN < 'r , 'h > { type Item = & 'h str ; # [inline] fn next (& mut self) -> Option < & 'h str > { self . it . next () . map (| span | & self . haystack [span]) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}

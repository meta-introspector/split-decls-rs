// Generated macro for impl_67 (impl)
macro_rules! Depcrate_regex_bytesimpl_67 {
() => {
// Module: crate::regex::bytes
// Provides: {"impl_67"}
// Dependencies: {}
impl < 'r , 'h > Iterator for Split < 'r , 'h > { type Item = & 'h [u8] ; # [inline] fn next (& mut self) -> Option < & 'h [u8] > { self . it . next () . map (| span | & self . haystack [span]) } }
};
}

// Generated macro for impl_70 (impl)
macro_rules! Depcrate_regex_bytesimpl_70 {
() => {
// Module: crate::regex::bytes
// Provides: {"impl_70"}
// Dependencies: {}
impl < 'r , 'h > Iterator for SplitN < 'r , 'h > { type Item = & 'h [u8] ; # [inline] fn next (& mut self) -> Option < & 'h [u8] > { self . it . next () . map (| span | & self . haystack [span]) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}

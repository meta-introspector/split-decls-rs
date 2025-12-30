// Generated macro for impl_138 (impl)
macro_rules! Depcrate_regex_stringimpl_138 {
() => {
// Module: crate::regex::string
// Provides: {"impl_138"}
// Dependencies: {}
impl < 'c , 'h > Iterator for SubCaptureMatches < 'c , 'h > { type Item = Option < Match < 'h > > ; # [inline] fn next (& mut self) -> Option < Option < Match < 'h > > > { self . it . next () . map (| group | { group . map (| sp | Match :: new (self . haystack , sp . start , sp . end)) }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } # [inline] fn count (self) -> usize { self . it . count () } }
};
}

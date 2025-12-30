// Generated macro for impl_122 (impl)
macro_rules! Depcrate_regex_stringimpl_122 {
() => {
// Module: crate::regex::string
// Provides: {"impl_122"}
// Dependencies: {}
impl < 'r , 'h > Iterator for Matches < 'r , 'h > { type Item = Match < 'h > ; # [inline] fn next (& mut self) -> Option < Match < 'h > > { self . it . next () . map (| sp | Match :: new (self . haystack , sp . start () , sp . end ())) } # [inline] fn count (self) -> usize { self . it . count () } }
};
}

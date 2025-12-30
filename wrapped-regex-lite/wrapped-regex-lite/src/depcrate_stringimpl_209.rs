// Generated macro for impl_209 (impl)
macro_rules! Depcrate_stringimpl_209 {
() => {
// Module: crate::string
// Provides: {"impl_209"}
// Dependencies: {}
impl < 'r , 'h > Iterator for Matches < 'r , 'h > { type Item = Match < 'h > ; # [inline] fn next (& mut self) -> Option < Match < 'h > > { self . it . next () . map (| (s , e) | Match :: new (self . haystack , s , e)) } # [inline] fn count (self) -> usize { self . it . count () } }
};
}

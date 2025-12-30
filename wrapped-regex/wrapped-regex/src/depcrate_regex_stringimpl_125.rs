// Generated macro for impl_125 (impl)
macro_rules! Depcrate_regex_stringimpl_125 {
() => {
// Module: crate::regex::string
// Provides: {"impl_125"}
// Dependencies: {}
impl < 'r , 'h > Iterator for CaptureMatches < 'r , 'h > { type Item = Captures < 'h > ; # [inline] fn next (& mut self) -> Option < Captures < 'h > > { let static_captures_len = self . it . regex () . static_captures_len () ; self . it . next () . map (| caps | Captures { haystack : self . haystack , caps , static_captures_len , }) } # [inline] fn count (self) -> usize { self . it . count () } }
};
}

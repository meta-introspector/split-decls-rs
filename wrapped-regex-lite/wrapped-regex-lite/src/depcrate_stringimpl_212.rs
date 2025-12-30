// Generated macro for impl_212 (impl)
macro_rules! Depcrate_stringimpl_212 {
() => {
// Module: crate::string
// Provides: {"impl_212"}
// Dependencies: {}
impl < 'r , 'h > Iterator for CaptureMatches < 'r , 'h > { type Item = Captures < 'h > ; # [inline] fn next (& mut self) -> Option < Captures < 'h > > { self . it . next () . map (| slots | Captures { haystack : self . haystack , slots : CaptureLocations (slots) , pikevm : Arc :: clone (& self . re . pikevm) , }) } # [inline] fn count (self) -> usize { self . it . count () } }
};
}

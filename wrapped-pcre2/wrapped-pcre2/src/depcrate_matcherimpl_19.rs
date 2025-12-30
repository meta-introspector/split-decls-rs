// Generated macro for impl_19 (impl)
macro_rules! Depcrate_matcherimpl_19 {
() => {
// Module: crate::matcher
// Provides: {"impl_19"}
// Dependencies: {}
impl Captures for RegexCaptures { fn len (& self) -> usize { self . locs . len () } fn get (& self , i : usize) -> Option < Match > { self . locs . get (i) . map (| (s , e) | Match :: new (s , e)) } }
};
}

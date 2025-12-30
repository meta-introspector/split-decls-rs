// Generated macro for impl_425 (impl)
macro_rules! Depcrate_util_prefilterimpl_425 {
() => {
// Module: crate::util::prefilter
// Provides: {"impl_425"}
// Dependencies: {}
# [cfg (all (feature = "std" , feature = "perf-literal"))] impl PrefilterI for Memmem { fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { use crate :: util :: primitives :: PatternID ; self . 0 . find (& haystack [span]) . map_or (Candidate :: None , | i | { let start = span . start + i ; let end = start + self . 0 . needle () . len () ; Candidate :: Match (Match :: new (PatternID :: ZERO , start .. end)) }) } }
};
}

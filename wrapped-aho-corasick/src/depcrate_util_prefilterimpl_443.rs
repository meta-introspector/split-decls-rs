// Generated macro for impl_443 (impl)
macro_rules! Depcrate_util_prefilterimpl_443 {
() => {
// Module: crate::util::prefilter
// Provides: {"impl_443"}
// Dependencies: {}
# [cfg (feature = "perf-literal")] impl PrefilterI for StartBytesOne { fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { memchr :: memchr (self . byte1 , & haystack [span]) . map (| i | span . start + i) . map_or (Candidate :: None , Candidate :: PossibleStartOfMatch) } }
};
}

// Generated macro for impl_447 (impl)
macro_rules! Depcrate_util_prefilterimpl_447 {
() => {
// Module: crate::util::prefilter
// Provides: {"impl_447"}
// Dependencies: {}
# [cfg (feature = "perf-literal")] impl PrefilterI for StartBytesThree { fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { memchr :: memchr3 (self . byte1 , self . byte2 , self . byte3 , & haystack [span]) . map (| i | span . start + i) . map_or (Candidate :: None , Candidate :: PossibleStartOfMatch) } }
};
}

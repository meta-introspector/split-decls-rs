// Generated macro for impl_445 (impl)
macro_rules! Depcrate_util_prefilterimpl_445 {
() => {
// Module: crate::util::prefilter
// Provides: {"impl_445"}
// Dependencies: {}
# [cfg (feature = "perf-literal")] impl PrefilterI for StartBytesTwo { fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { memchr :: memchr2 (self . byte1 , self . byte2 , & haystack [span]) . map (| i | span . start + i) . map_or (Candidate :: None , Candidate :: PossibleStartOfMatch) } }
};
}

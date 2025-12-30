// Generated macro for impl_437 (impl)
macro_rules! Depcrate_util_prefilterimpl_437 {
() => {
// Module: crate::util::prefilter
// Provides: {"impl_437"}
// Dependencies: {}
# [cfg (feature = "perf-literal")] impl PrefilterI for RareBytesTwo { fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { memchr :: memchr2 (self . byte1 , self . byte2 , & haystack [span]) . map (| i | { let pos = span . start + i ; let offset = self . offsets . set [usize :: from (haystack [pos])] . max ; cmp :: max (span . start , pos . saturating_sub (usize :: from (offset))) }) . map_or (Candidate :: None , Candidate :: PossibleStartOfMatch) } }
};
}

// Generated macro for impl_841 (impl)
macro_rules! Depcrate_util_prefilter_memchrimpl_841 {
() => {
// Module: crate::util::prefilter::memchr
// Provides: {"impl_841"}
// Dependencies: {}
impl PrefilterI for Memchr2 { fn find (& self , haystack : & [u8] , span : Span) -> Option < Span > { # [cfg (not (feature = "perf-literal-substring"))] { unreachable ! () } # [cfg (feature = "perf-literal-substring")] { memchr :: memchr2 (self . 0 , self . 1 , & haystack [span]) . map (| i | { let start = span . start + i ; let end = start + 1 ; Span { start , end } }) } } fn prefix (& self , haystack : & [u8] , span : Span) -> Option < Span > { let b = * haystack . get (span . start) ? ; if self . 0 == b || self . 1 == b { Some (Span { start : span . start , end : span . start + 1 }) } else { None } } fn memory_usage (& self) -> usize { 0 } fn is_fast (& self) -> bool { true } }
};
}

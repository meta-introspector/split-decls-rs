// Generated macro for impl_828 (impl)
macro_rules! Depcrate_util_prefilter_aho_corasickimpl_828 {
() => {
// Module: crate::util::prefilter::aho_corasick
// Provides: {"impl_828"}
// Dependencies: {}
impl PrefilterI for AhoCorasick { fn find (& self , haystack : & [u8] , span : Span) -> Option < Span > { # [cfg (not (feature = "perf-literal-multisubstring"))] { unreachable ! () } # [cfg (feature = "perf-literal-multisubstring")] { let input = aho_corasick :: Input :: new (haystack) . span (span . start .. span . end) ; self . ac . find (input) . map (| m | Span { start : m . start () , end : m . end () }) } } fn prefix (& self , haystack : & [u8] , span : Span) -> Option < Span > { # [cfg (not (feature = "perf-literal-multisubstring"))] { unreachable ! () } # [cfg (feature = "perf-literal-multisubstring")] { let input = aho_corasick :: Input :: new (haystack) . anchored (aho_corasick :: Anchored :: Yes) . span (span . start .. span . end) ; self . ac . find (input) . map (| m | Span { start : m . start () , end : m . end () }) } } fn memory_usage (& self) -> usize { # [cfg (not (feature = "perf-literal-multisubstring"))] { unreachable ! () } # [cfg (feature = "perf-literal-multisubstring")] { self . ac . memory_usage () } } fn is_fast (& self) -> bool { # [cfg (not (feature = "perf-literal-multisubstring"))] { unreachable ! () } # [cfg (feature = "perf-literal-multisubstring")] { false } } }
};
}

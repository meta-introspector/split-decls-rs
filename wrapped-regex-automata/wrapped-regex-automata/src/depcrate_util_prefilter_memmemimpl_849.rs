// Generated macro for impl_849 (impl)
macro_rules! Depcrate_util_prefilter_memmemimpl_849 {
() => {
// Module: crate::util::prefilter::memmem
// Provides: {"impl_849"}
// Dependencies: {}
impl PrefilterI for Memmem { fn find (& self , haystack : & [u8] , span : Span) -> Option < Span > { # [cfg (not (all (feature = "std" , feature = "perf-literal-substring")))] { unreachable ! () } # [cfg (all (feature = "std" , feature = "perf-literal-substring"))] { self . finder . find (& haystack [span]) . map (| i | { let start = span . start + i ; let end = start + self . finder . needle () . len () ; Span { start , end } }) } } fn prefix (& self , haystack : & [u8] , span : Span) -> Option < Span > { # [cfg (not (all (feature = "std" , feature = "perf-literal-substring")))] { unreachable ! () } # [cfg (all (feature = "std" , feature = "perf-literal-substring"))] { let needle = self . finder . needle () ; if haystack [span] . starts_with (needle) { Some (Span { end : span . start + needle . len () , .. span }) } else { None } } } fn memory_usage (& self) -> usize { # [cfg (not (all (feature = "std" , feature = "perf-literal-substring")))] { unreachable ! () } # [cfg (all (feature = "std" , feature = "perf-literal-substring"))] { self . finder . needle () . len () } } fn is_fast (& self) -> bool { # [cfg (not (all (feature = "std" , feature = "perf-literal-substring")))] { unreachable ! () } # [cfg (all (feature = "std" , feature = "perf-literal-substring"))] { true } } }
};
}

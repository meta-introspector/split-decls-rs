// Generated macro for impl_863 (impl)
macro_rules! Depcrate_util_prefilterimpl_863 {
() => {
// Module: crate::util::prefilter
// Provides: {"impl_863"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < P : PrefilterI + ? Sized > PrefilterI for Arc < P > { # [cfg_attr (feature = "perf-inline" , inline (always))] fn find (& self , haystack : & [u8] , span : Span) -> Option < Span > { (* * self) . find (haystack , span) } # [cfg_attr (feature = "perf-inline" , inline (always))] fn prefix (& self , haystack : & [u8] , span : Span) -> Option < Span > { (* * self) . prefix (haystack , span) } # [cfg_attr (feature = "perf-inline" , inline (always))] fn memory_usage (& self) -> usize { (* * self) . memory_usage () } # [cfg_attr (feature = "perf-inline" , inline (always))] fn is_fast (& self) -> bool { (& * * self) . is_fast () } }
};
}
